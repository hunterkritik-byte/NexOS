use x86_64::structures::idt::InterruptDescriptorTable;

pub struct Interrupts {
    pub idt: InterruptDescriptorTable,
}

impl Interrupts {
    pub fn new() -> Self {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        Self { idt }
    }

    pub fn load(&'static self) { self.idt.load(); }
}

extern "x86-interrupt" fn breakpoint_handler(_stack: x86_64::structures::idt::InterruptStackFrame) {
    // Kernel breakpoint hook. Keep handler minimal until scheduler/console locking exists.
}

extern "x86-interrupt" fn page_fault_handler(
    _stack: x86_64::structures::idt::InterruptStackFrame,
    _error: x86_64::structures::idt::PageFaultErrorCode,
) {
    // A production kernel should terminate the current process here rather than
    // touching user memory from interrupt context.
    loop { core::hint::spin_loop(); }
}
