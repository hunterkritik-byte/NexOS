#![no_std]
#![no_main]

const SYS_WRITE:usize=1;
const SYS_YIELD:usize=2;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo)->!{loop{core::hint::spin_loop()}}

#[unsafe(no_mangle)]
pub extern "C" fn _start()->! {
    write(b"NexOS userspace shell\n");
    write(b"nexsh$ ");
    loop { syscall(SYS_YIELD,0,0,0); }
}

fn write(s:&[u8]){
    let _=syscall(SYS_WRITE,s.as_ptr() as usize,s.len(),0);
}
#[inline(always)]
fn syscall(n:usize,a:usize,b:usize,c:usize)->usize{
    let ret:usize;
    unsafe { core::arch::asm!("syscall",in("rax") n,in("rdi") a,in("rsi") b,in("rdx") c,lateout("rax") ret,clobber_abi("system")); }
    ret
}
