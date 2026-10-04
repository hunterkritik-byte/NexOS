#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessError { InvalidElf, Unsupported, NoMemory }

pub struct UserProcess {
    pub entry: u64,
    pub stack_top: u64,
}

pub fn find_init_elf(_ramdisk: &[u8]) -> Result<&[u8], ProcessError> {
    Err(ProcessError::InvalidElf)
}

pub unsafe fn build_elf_process(
    _physical_offset: x86_64::VirtAddr,
    _active: &x86_64::structures::paging::PageTable,
    _allocator: &mut impl x86_64::structures::paging::FrameAllocator<Size4KiB>,
    _image: &[u8],
) -> Result<UserProcess, ProcessError> {
    Err(ProcessError::Unsupported)
}

pub struct Process { pub entry: usize, pub stack_top: usize }
impl Process { pub const fn new(entry: usize, stack_top: usize) -> Self { Self { entry, stack_top } } }

pub struct Scheduler { slots: [Option<Process>; 8] }
impl Scheduler {
    pub const fn new() -> Self { Self { slots: [None; 8] } }
    pub fn add(&mut self, process: Process) -> Option<usize> {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() { *slot=Some(process); return Some(i); }
        }
        None
    }
}

pub unsafe fn launch_user_process(_process: UserProcess) -> ! {
    loop { core::hint::spin_loop(); }
}
