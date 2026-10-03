#[repr(C)]
pub struct Registers {
    pub r9: u64,
    pub r8: u64,
    pub r10: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rax: u64,
}

pub const SYS_EXIT: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_YIELD: u64 = 2;
pub const SYS_GETPID: u64 = 3;

core::arch::global_asm!(
    ".globl nexos_int80_entry",
    "nexos_int80_entry:",
    "cld",
    "swapgs",
    "push rax",
    "push rdi",
    "push rsi",
    "push rdx",
    "push r10",
    "push r8",
    "push r9",
    "mov rdi, rsp",
    "call {dispatch}",
    "mov r12, rax",
    "pop r9",
    "pop r8",
    "pop r10",
    "pop rdx",
    "pop rsi",
    "pop rdi",
    "pop rax",
    "mov rax, r12",
    "swapgs",
    "iretq",
    dispatch = sym syscall_dispatch_entry,
);

#[unsafe(no_mangle)]
extern "C" fn syscall_dispatch_entry(regs: *const Registers) -> u64 {
    if regs.is_null() { return u64::MAX; }
    unsafe { dispatch(&*regs, 1) }
}

pub fn dispatch(regs: &Registers, pid: u64) -> u64 {
    match regs.rax {
        SYS_GETPID => pid,
        SYS_YIELD => 0,
        SYS_WRITE => regs.rsi,
        SYS_EXIT => 0,
        _ => u64::MAX,
    }
}

pub unsafe fn install_syscall_gate(
    idt: &mut x86_64::structures::idt::InterruptDescriptorTable,
) {
    let entry = idt[0x80].set_handler_addr(
        x86_64::VirtAddr::new(nexos_int80_entry as usize as u64)
    );
    entry.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
}
