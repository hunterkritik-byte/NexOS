use x86_64::{structures::paging::{PageTable, PageTableFlags, PhysFrame, Size4KiB}, PhysAddr, VirtAddr};

pub const USER_CODE_BASE: u64 = 0x0000_0040_0000;
pub const USER_STACK_TOP: u64 = 0x0000_0080_0000;

pub struct UserAddressSpace {
    pub root: PhysFrame<Size4KiB>,
}

impl UserAddressSpace {
    pub const fn new(root: PhysFrame<Size4KiB>) -> Self { Self { root } }

    pub fn flags() -> PageTableFlags {
        PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE
    }

    /// Address-space switching primitive. A real allocator/page-table builder must
    /// supply the physical frame for the user's PML4.
    pub unsafe fn activate(&self) {
        let phys = self.root.start_address().as_u64();
        core::arch::asm!("mov cr3, {}", in(reg) phys, options(nostack, preserves_flags));
    }
}
