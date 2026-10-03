use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::{
    PhysAddr,
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
};

pub struct BootInfoFrameAllocator<'a> {
    regions: &'a MemoryRegions,
    next_region: usize,
    next_addr: u64,
}

impl<'a> BootInfoFrameAllocator<'a> {
    pub fn new(regions: &'a MemoryRegions) -> Self {
        Self { regions, next_region: 0, next_addr: 0 }
    }

    fn next_usable_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        loop {
            if self.next_region >= self.regions.len() {
                return None;
            }
            let region = &self.regions[self.next_region];
            if region.kind != MemoryRegionKind::Usable {
                self.next_region += 1;
                self.next_addr = 0;
                continue;
            }

            let mut addr = if self.next_addr == 0 {
                (region.start + 4095) & !4095
            } else {
                self.next_addr
            };

            if addr + 4096 <= region.end {
                self.next_addr = addr + 4096;
                return Some(PhysFrame::containing_address(PhysAddr::new(addr)));
            }

            self.next_region += 1;
            self.next_addr = 0;
        }
    }
}

unsafe impl<'a> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.next_usable_frame()
    }
}
