#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UserContext {
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
}

impl UserContext {
    pub const fn new(entry: u64, stack: u64) -> Self {
        Self {
            rip: entry,
            rsp: stack,
            rflags: 0x202,
            cs: 0x23,
            ss: 0x1b,
        }
    }
}

/// Enter ring 3 through an iretq frame. GDT selectors must match the kernel's
/// user-code/user-data descriptors before this is called.
pub unsafe fn enter_user(ctx: &UserContext) -> ! {
    core::arch::asm!(
        "push {ss}",
        "push {rsp}",
        "push {rflags}",
        "push {cs}",
        "push {rip}",
        "iretq",
        ss = in(reg) ctx.ss,
        rsp = in(reg) ctx.rsp,
        rflags = in(reg) ctx.rflags,
        cs = in(reg) ctx.cs,
        rip = in(reg) ctx.rip,
        options(noreturn)
    );
}
