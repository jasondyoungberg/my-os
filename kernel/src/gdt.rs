use alloc::vec::Vec;
use spin::Lazy;
use x86_64::structures::{
    gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    tss::TaskStateSegment,
};

use crate::SMP_RESPONSE;

pub const KERNEL_CODE_SELECTOR: u16 = 1 << 3;
pub const KERNEL_DATA_SELECTOR: u16 = 2 << 3;
pub const USER_DATA_SELECTOR: u16 = 3 << 3 | 3;
pub const USER_CODE_SELECTOR: u16 = 4 << 3 | 3;

static GDT: Lazy<GdtInfo> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::empty();

    let kernel_code = gdt.append(Descriptor::kernel_code_segment());
    let kernel_data = gdt.append(Descriptor::kernel_data_segment());
    let user_data = gdt.append(Descriptor::user_data_segment());
    let user_code = gdt.append(Descriptor::user_code_segment());

    let tss = TSS
        .iter()
        .map(|tss| gdt.append(Descriptor::tss_segment(tss)))
        .collect();

    GdtInfo {
        gdt,
        kernel_code,
        kernel_data,
        user_data,
        user_code,
        tss,
    }
});

static TSS: Lazy<Vec<TaskStateSegment>> = Lazy::new(|| {
    let cpu_count = SMP_RESPONSE.cpus().len();
    (0..cpu_count).map(|_| TaskStateSegment::new()).collect()
});

struct GdtInfo {
    gdt: GlobalDescriptorTable<69>,
    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
    user_data: SegmentSelector,
    user_code: SegmentSelector,
    tss: Vec<SegmentSelector>,
}

pub fn init() {}
