use x86_64::{VirtAddr,structures::paging::{Page,PageTableFlags,Mapper,FrameAllocator,Size4KiB,PhysFrame}};

pub const USER_ENTRY:u64=0x0040_0000;
pub const USER_STACK_TOP:u64=0x0080_0000;
pub const USER_STACK_PAGES:u64=8;

pub struct UserProcess{pub root:PhysFrame<Size4KiB>,pub entry:u64,pub stack_top:u64}

pub fn build_user_image<M,A>(mapper:&mut M,allocator:&mut A,image:&[u8],entry:u64)->Result<UserProcess,&'static str>
where M:Mapper<Size4KiB>,A:FrameAllocator<Size4KiB>{
 if image.is_empty(){return Err("empty user image")}
 let pages=((image.len() as u64)+4095)/4096;
 let root=allocator.allocate_frame().ok_or("out of physical memory")?;
 let flags=PageTableFlags::PRESENT|PageTableFlags::USER_ACCESSIBLE;
 for i in 0..pages{
  let frame=allocator.allocate_frame().ok_or("out of physical memory")?;
  let page=Page::containing_address(VirtAddr::new(USER_ENTRY+i*4096));
  unsafe{mapper.map_to(page,frame,flags|PageTableFlags::WRITABLE,allocator).map_err(|_|"map user image")?.flush();}
  let n=((image.len() as u64-i*4096).min(4096)) as usize;
  unsafe{core::ptr::copy_nonoverlapping(image.as_ptr().add((i*4096) as usize),frame.start_address().as_u64() as *mut u8,n);}
 }
 for i in 0..USER_STACK_PAGES{
  let addr=USER_STACK_TOP-(i+1)*4096;
  let frame=allocator.allocate_frame().ok_or("out of physical memory")?;
  let page=Page::containing_address(VirtAddr::new(addr));
  unsafe{mapper.map_to(page,frame,flags|PageTableFlags::WRITABLE,allocator).map_err(|_|"map user stack")?.flush();}
 }
 Ok(UserProcess{root,entry,stack_top:USER_STACK_TOP-16})
}

pub unsafe fn install_user_cr3(root:PhysFrame<Size4KiB>){
 core::arch::asm!("mov cr3,{}",in(reg)root.start_address().as_u64(),options(nostack,preserves_flags));
}
