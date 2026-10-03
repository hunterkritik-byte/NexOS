#[repr(C)]
pub struct SyscallFrame {
    pub rax: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub r10: u64,
    pub r8: u64,
    pub r9: u64,
}

/// Stable syscall ABI for the first userspace prototype.
///
/// The CPU entry mechanism is intentionally installed only after the GDT/IDT,
/// TSS, and per-CPU syscall MSRs are configured.
pub const SYS_EXIT: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_YIELD: u64 = 2;
pub const SYS_GETPID: u64 = 3;

pub fn dispatch(frame: &SyscallFrame, pid: u64) -> u64 {
    match frame.rax {
        SYS_EXIT => 0,
        SYS_WRITE => frame.rdx,
        SYS_YIELD => 0,
        SYS_GETPID => pid,
        _ => u64::MAX,
    }
}
