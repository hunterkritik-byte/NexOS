use x86_64::structures::idt::InterruptDescriptorTable;
use super::syscall;

pub struct Interrupts {
    pub idt: InterruptDescriptorTable,
}

impl Interrupts {
    pub fn new() -> Self {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe { syscall::install_syscall_gate(&mut idt); }
        Self { idt }
    }

    pub fn load(&'static self) { self.idt.load(); }
}

extern "x86-interrupt" fn breakpoint_handler(_stack: x86_64::structures::idt::InterruptStackFrame) {}

extern "x86-interrupt" fn page_fault_handler(
    _stack: x86_64::structures::idt::InterruptStackFrame,
    _error: x86_64::structures::idt::PageFaultErrorCode,
) {
    loop { core::hint::spin_loop(); }
}
