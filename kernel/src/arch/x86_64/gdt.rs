use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{
    instructions::segmentation::Segment,
    VirtAddr,
    instructions::segmentation::CS,
    instructions::tables::load_tss,
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const USER_CODE_SELECTOR: u16 = 0x1b;
pub const USER_DATA_SELECTOR: u16 = 0x23;

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
    pub fn new(kernel_stack_top: u64) -> Self {
        let mut tss = TaskStateSegment::new();
        tss.privilege_stack_table[0] = VirtAddr::new(kernel_stack_top);

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
        load_tss(self.tss_selector);
    }
}
