use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{VirtAddr, instructions::segmentation::{CS, SS}, instructions::tables::load_tss};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct GlobalTables {
    pub gdt: GlobalDescriptorTable,
    pub code: SegmentSelector,
    pub data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: TaskStateSegment,
}

impl GlobalTables {
    pub fn new() -> Self {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] =
            VirtAddr::new(0x0000_0000_0080_0000);

        let mut gdt = GlobalDescriptorTable::new();
        let code = gdt.append(Descriptor::kernel_code_segment());
        let data = gdt.append(Descriptor::kernel_data_segment());
        let user_code = gdt.append(Descriptor::user_code_segment());
        let user_data = gdt.append(Descriptor::user_data_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&tss));

        // TSS selector is retained in the descriptor table; loading it is
        // performed by init() after the GDT has been loaded.
        let _ = tss_selector;
        Self { gdt, code, data, user_code, user_data, tss }
    }

    pub unsafe fn init(&self) {
        self.gdt.load();
        CS::set_reg(self.code);
        SS::set_reg(self.data);
        load_tss(self.gdt.add_entry(Descriptor::tss_segment(&self.tss)));
    }
}
