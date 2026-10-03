use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{VirtAddr, instructions::segmentation::{CS, SS}, instructions::tables::load_tss};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const USER_CODE_SELECTOR: u16 = 0x1b;
pub const USER_DATA_SELECTOR: u16 = 0x23;

#[repr(align(16))]
static mut KERNEL_SYSCALL_STACK: [u8; 8192] = [0; 8192];

pub struct GlobalTables {
    pub gdt: GlobalDescriptorTable,
    pub code: SegmentSelector,
    pub data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss_selector: SegmentSelector,
    pub tss: TaskStateSegment,
}

impl GlobalTables {
    pub fn new() -> Self {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] =
            VirtAddr::new(0x0000_0000_0100_0000);
        let stack_top = unsafe { core::ptr::addr_of!(KERNEL_SYSCALL_STACK) as u64 + 8192 };
        tss.privilege_stack_table[0] = VirtAddr::new(stack_top);

        let mut gdt = GlobalDescriptorTable::new();
        let code = gdt.append(Descriptor::kernel_code_segment());
        let data = gdt.append(Descriptor::kernel_data_segment());
        let user_code = gdt.append(Descriptor::user_code_segment());
        let user_data = gdt.append(Descriptor::user_data_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&tss));
        Self { gdt, code, data, user_code, user_data, tss_selector, tss }
    }

    pub unsafe fn init(&'static self) {
        self.gdt.load();
        CS::set_reg(self.code);
        SS::set_reg(self.data);
        load_tss(self.tss_selector);
    }
}
