#[repr(C)]
pub struct Registers {
    pub rax: u64, pub rdi: u64, pub rsi: u64, pub rdx: u64,
    pub r10: u64, pub r8: u64, pub r9: u64,
}

pub const SYS_GETPID: u64 = 3;
pub const SYS_YIELD: u64 = 2;
pub const SYS_WRITE: u64 = 1;
pub const SYS_EXIT: u64 = 0;

pub fn dispatch(regs: &Registers, pid: u64) -> u64 {
    match regs.rax {
        SYS_GETPID => pid,
        SYS_YIELD => 0,
        SYS_WRITE => regs.rdx,
        SYS_EXIT => 0,
        _ => u64::MAX,
    }
}
