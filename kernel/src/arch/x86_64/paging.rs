use x86_64::structures::paging::{PageTable, PageTableFlags, PhysFrame, Size4KiB};
use x86_64::VirtAddr;

pub const KERNEL_FLAGS: PageTableFlags =
    PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
pub const USER_FLAGS: PageTableFlags =
    PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
pub const USER_CODE_FLAGS: PageTableFlags =
    PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
pub const USER_DATA_FLAGS: PageTableFlags =
    PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

pub struct AddressSpace {
    pub root: PhysFrame<Size4KiB>,
}

impl AddressSpace {
    pub const fn new(root: PhysFrame<Size4KiB>) -> Self { Self { root } }

    pub fn user_code_address(entry: u64) -> VirtAddr { VirtAddr::new(entry) }
    pub fn user_stack_address(top: u64) -> VirtAddr { VirtAddr::new(top) }

    pub unsafe fn activate(&self) {
        core::arch::asm!("mov cr3, {}", in(reg) self.root.start_address().as_u64(),
            options(nostack, preserves_flags));
    }
}
