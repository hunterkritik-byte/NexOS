pub mod boot_memory;
pub mod frame;
pub mod gdt;
pub mod idt;
pub mod paging;
pub mod syscall;

use core::mem::MaybeUninit;

static mut GDT_STORAGE: MaybeUninit<gdt::GlobalTables> = MaybeUninit::uninit();
static mut IDT_STORAGE: MaybeUninit<idt::Interrupts> = MaybeUninit::uninit();

/// Install the architectural tables before any attempt to enter ring 3.
pub unsafe fn init_cpu_tables() {
    GDT_STORAGE.write(gdt::GlobalTables::new());
    let gdt_ref = &*GDT_STORAGE.as_ptr();
    gdt_ref.init();

    IDT_STORAGE.write(idt::Interrupts::new());
    let idt_ref = &*IDT_STORAGE.as_ptr();
    idt_ref.load();
}
