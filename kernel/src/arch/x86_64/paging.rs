use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
        OffsetPageTable, mapper::MapToError,
    },
};

pub fn kernel_flags() -> PageTableFlags {
    PageTableFlags::PRESENT | PageTableFlags::WRITABLE
}
pub fn user_code_flags() -> PageTableFlags {
    PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE
}
pub fn user_data_flags() -> PageTableFlags {
    PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE
}

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let level_4 = x86_64::registers::control::Cr3::read().0;
    let phys = level_4.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    &mut *(virt.as_mut_ptr())
}

pub unsafe fn init_mapper(
    physical_memory_offset: VirtAddr,
) -> OffsetPageTable<'static> {
    let table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(table, physical_memory_offset)
}

pub struct AddressSpace {
    pub root: PhysFrame<Size4KiB>,
}

impl AddressSpace {
    pub const fn new(root: PhysFrame<Size4KiB>) -> Self { Self { root } }

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
        unsafe { mapper.map_to(page, frame, flags, allocator)?.flush(); }
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
