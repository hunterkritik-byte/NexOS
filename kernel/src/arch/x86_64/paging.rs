use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTable, PageTableFlags,
        PhysFrame, Size4KiB,
    },
    VirtAddr,
};

pub const KERNEL_FLAGS: PageTableFlags =
    PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
pub const USER_CODE_FLAGS: PageTableFlags =
    PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;
pub const USER_DATA_FLAGS: PageTableFlags =
    PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::USER_ACCESSIBLE;

pub struct AddressSpace {
    pub root: PhysFrame<Size4KiB>,
}

impl AddressSpace {
    pub const fn new(root: PhysFrame<Size4KiB>) -> Self {
        Self { root }
    }

    pub fn map_user_page<M, A>(
        &self,
        mapper: &mut M,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
        allocator: &mut A,
    ) -> Result<(), MapToError<Size4KiB>>
    where
        M: Mapper<Size4KiB>,
        A: FrameAllocator<Size4KiB>,
    {
        unsafe { mapper.map_to(page, frame, flags, allocator)?.flush() };
        Ok(())
    }

    pub fn user_page(address: u64) -> Page<Size4KiB> {
        Page::containing_address(VirtAddr::new(address))
    }

    pub unsafe fn activate(&self) {
        core::arch::asm!(
            "mov cr3, {}",
            in(reg) self.root.start_address().as_u64(),
            options(nostack, preserves_flags)
        );
    }
}
