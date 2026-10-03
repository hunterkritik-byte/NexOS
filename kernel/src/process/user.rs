use x86_64::{VirtAddr,PhysAddr,structures::paging::{Page,PageTableFlags,Mapper,FrameAllocator,Size4KiB,PhysFrame,PageTable,OffsetPageTable}};

pub const USER_ENTRY:u64=0x0040_0000;
pub const USER_STACK_TOP:u64=0x0080_0000;
pub const USER_STACK_PAGES:u64=8;

pub struct UserProcess{pub root:PhysFrame<Size4KiB>,pub entry:u64,pub stack_top:u64}

/// Creates a separate PML4, preserves the kernel half of the active address space,
/// and maps the user image/stack only in USER_ACCESSIBLE pages.
pub unsafe fn build_user_address_space<A>(
 physical_offset:VirtAddr, active:&PageTable, allocator:&mut A,
 image:&[u8], entry:u64
)->Result<UserProcess,&'static str>
where A:FrameAllocator<Size4KiB>{
 if image.is_empty(){return Err("empty user image")}
 if entry<USER_ENTRY||entry>=0x0000_8000_0000_0000{return Err("invalid user entry")}
 let root=allocator.allocate_frame().ok_or("out of physical memory")?;
 let root_virt=physical_offset+root.start_address().as_u64();
 let root_table=&mut *(root_virt.as_mut_ptr::<PageTable>());
 root_table.zero();
 // Preserve only kernel-space PML4 entries. User entries remain empty.
 for i in 256..512 { root_table[i]=active[i].clone(); }
 let mut mapper=OffsetPageTable::new(root_table,physical_offset);
 let flags=PageTableFlags::PRESENT|PageTableFlags::USER_ACCESSIBLE|PageTableFlags::WRITABLE;
 let pages=((image.len() as u64)+4095)/4096;
 for i in 0..pages{
  let frame=allocator.allocate_frame().ok_or("out of physical memory")?;
  let page=Page::containing_address(VirtAddr::new(USER_ENTRY+i*4096));
  mapper.map_to(page,frame,flags,allocator).map_err(|_|"map user image")?.flush();
  let n=((image.len() as u64-i*4096).min(4096)) as usize;
  core::ptr::copy_nonoverlapping(
   image.as_ptr().add((i*4096) as usize),
   (physical_offset+frame.start_address().as_u64()).as_mut_ptr::<u8>(),n);
 }
 for i in 0..USER_STACK_PAGES{
  let addr=USER_STACK_TOP-(i+1)*4096;
  let frame=allocator.allocate_frame().ok_or("out of physical memory")?;
  let page=Page::containing_address(VirtAddr::new(addr));
  mapper.map_to(page,frame,flags,allocator).map_err(|_|"map user stack")?.flush();
 }
 Ok(UserProcess{root,entry,stack_top:USER_STACK_TOP-16})
}

pub unsafe fn install_user_cr3(root:PhysFrame<Size4KiB>){
 core::arch::asm!("mov cr3,{}",in(reg)root.start_address().as_u64(),options(nostack,preserves_flags));
}

pub fn launch_user_process(process: UserProcess) -> ! {
    let context = crate::x86_kernel::process::context::UserContext::new(
        process.entry,
        process.stack_top,
    );
    unsafe {
        install_user_cr3(process.root);
        crate::x86_kernel::process::context::enter_user(&context);
    }
}
