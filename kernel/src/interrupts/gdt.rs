use spin::Lazy;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, SS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

use crate::interrupts::TSS;

pub static GDT_INFO: Lazy<GdtInfo> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let kernel_code_selector = gdt.append(Descriptor::kernel_code_segment());
    let kernel_data_selector = gdt.append(Descriptor::kernel_data_segment());
    let user_data_selector = gdt.append(Descriptor::user_data_segment());
    let user_code_selector = gdt.append(Descriptor::user_code_segment());
    let tss = gdt.append(Descriptor::tss_segment(&TSS));

    GdtInfo {
        gdt,
        kernel_code_selector,
        kernel_data_selector,
        user_code_selector,
        user_data_selector,
        tss,
    }
});

pub fn load() {
    GDT_INFO.gdt.load();
    unsafe {
        CS::set_reg(GDT_INFO.kernel_code_selector);
        SS::set_reg(GDT_INFO.kernel_data_selector);
        load_tss(self::GDT_INFO.tss);
    }
}

#[allow(dead_code)] // TODO: remove this later
pub struct GdtInfo {
    pub gdt: GlobalDescriptorTable,
    pub kernel_code_selector: SegmentSelector,
    pub kernel_data_selector: SegmentSelector,
    pub user_code_selector: SegmentSelector,
    pub user_data_selector: SegmentSelector,
    pub tss: SegmentSelector,
}
