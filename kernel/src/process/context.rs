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
    /// Selectors correspond to the GDT layout installed by arch::x86_64::gdt:
    /// kernel code 0x08, kernel data 0x10, user code 0x1b, user data 0x23.
    pub const fn new(entry: u64, stack: u64) -> Self {
        Self {
            rip: entry,
            rsp: stack,
            rflags: 0x202,
            cs: 0x1b,
            ss: 0x23,
        }
    }
}

/// Enter ring 3 through a hardware iretq frame.
///
/// The caller must have installed the matching GDT/TSS and activated the
/// process CR3 before invoking this function.
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
