use spin::Lazy;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

pub static TSS: TaskStateSegment = TaskStateSegment::new();

pub static GDT: Lazy<GdtInfo> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let kernel_code = gdt.append(Descriptor::kernel_code_segment());
    let kernel_data = gdt.append(Descriptor::kernel_data_segment());
    let user_data = gdt.append(Descriptor::user_data_segment());
    let user_code = gdt.append(Descriptor::user_code_segment());
    let tss = gdt.append(Descriptor::tss_segment(&TSS));

    GdtInfo {
        gdt,
        kernel_code,
        kernel_data,
        user_code,
        user_data,
        tss,
    }
});

pub struct GdtInfo {
    pub gdt: GlobalDescriptorTable,
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}

pub fn init() {
    GDT.gdt.load();
    unsafe {
        CS::set_reg(GDT.kernel_code);
        SS::set_reg(GDT.kernel_data);
        load_tss(GDT.tss);
    }
}
