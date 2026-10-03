use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::{PhysAddr, structures::paging::{PhysFrame, Size4KiB}};

const FRAME_SIZE: u64 = 4096;
static NEXT_FRAME: AtomicU64 = AtomicU64::new(0);

pub struct FrameAllocator {
    pub start: u64,
    pub end: u64,
}

impl FrameAllocator {
    pub const fn new(start: u64, end: u64) -> Self { Self { start, end } }

    pub fn alloc(&self) -> Option<PhysFrame<Size4KiB>> {
        let current = NEXT_FRAME.fetch_add(FRAME_SIZE, Ordering::Relaxed) + self.start;
        if current >= self.end { return None; }
        PhysFrame::containing_address(PhysAddr::new(current)).into()
    }
}
