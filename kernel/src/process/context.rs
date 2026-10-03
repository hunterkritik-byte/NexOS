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
        Self { rip: entry, rsp: stack, rflags: 0x202, cs: 0x1b, ss: 0x23 }
    }
}

pub unsafe fn enter_user(ctx: &UserContext) -> ! {
    core::arch::asm!(
        "push {ss}",
        "push {rsp}",
        "push {rflags}",
        "push {cs}",
        "push {rip}",
        "iretq",
        ss=in(reg)ctx.ss, rsp=in(reg)ctx.rsp, rflags=in(reg)ctx.rflags,
        cs=in(reg)ctx.cs, rip=in(reg)ctx.rip, options(noreturn)
    );
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KernelContext {
    pub rsp: u64,
    pub rip: u64,
    pub cr3: u64,
}

pub unsafe fn switch_kernel_context(from: &mut KernelContext, to: &KernelContext) -> ! {
    core::arch::asm!(
        "mov [rdi], rsp",
        "lea rax, [rip + 2f]",
        "mov [rdi + 8], rax",
        "mov rsp, [rsi]",
        "mov cr3, [rsi + 16]",
        "push qword ptr [rsi + 8]",
        "ret",
        "2:",
        in("rdi") from,
        in("rsi") to,
        out("rax") _,
        options(noreturn)
    );
}
