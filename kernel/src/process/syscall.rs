pub const SYS_EXIT: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_YIELD: usize = 2;
pub const SYS_GETPID: usize = 3;

#[derive(Clone, Copy)]
pub struct SyscallContext {
    pub number: usize,
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
}

pub fn dispatch(context: SyscallContext, pid: usize) -> usize {
    match context.number {
        SYS_EXIT => 0,
        SYS_WRITE => context.arg2,
        SYS_YIELD => 0,
        SYS_GETPID => pid,
        _ => usize::MAX,
    }
}
