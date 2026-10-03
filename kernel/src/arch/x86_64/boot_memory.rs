use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::{PhysAddr, structures::paging::{FrameAllocator, PhysFrame, Size4KiB}};

/// Frame allocator backed by the bootloader-provided physical memory map.
/// Only USABLE regions are handed to the kernel; firmware, bootloader and
/// reserved regions are never allocated.
pub struct BootInfoFrameAllocator<'a> {
    regions: &'a MemoryRegions,
    next_region: usize,
    next_addr: Option<u64>,
}

impl<'a> BootInfoFrameAllocator<'a> {
    pub fn new(regions: &'a MemoryRegions) -> Self {
        Self { regions, next_region: 0, next_addr: None }
    }

    fn next_usable_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        loop {
            if self.next_region >= self.regions.len() {
                return None;
            }

            let region = &self.regions[self.next_region];
            let start = region.start;
            let end = region.end;

            if region.kind != MemoryRegionKind::Usable {
                self.next_region += 1;
                self.next_addr = None;
                continue;
            }

            let addr = self.next_addr.unwrap_or(start);
            if addr + 4096 <= end {
                self.next_addr = Some(addr + 4096);
                return Some(PhysFrame::containing_address(PhysAddr::new(addr)));
            }

            self.next_region += 1;
            self.next_addr = None;
        }
    }
}

unsafe impl<'a> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.next_usable_frame()
    }
}
