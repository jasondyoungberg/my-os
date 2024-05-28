use alloc::vec::Vec;
use spin::Lazy;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

use crate::{gsdata::CpuId, mapper::create_ministack, SMP_RESPONSE};

const STACK_SIZE: u64 = 64 * 1024; // 64 KiB

const MAX_CORES: usize = 16; // Maximum number of cores supported by the kernel
const GDT_SIZE: usize = 6 + 2 * MAX_CORES;

pub static TSS: Lazy<Vec<TaskStateSegment>> = Lazy::new(|| {
    let cpu_cnt = SMP_RESPONSE.cpus().len();
    (0..cpu_cnt)
        .map(|_| {
            let mut tss = TaskStateSegment::new();

            for i in 0..7 {
                tss.interrupt_stack_table[i] = create_ministack(STACK_SIZE);
            }

            for i in 0..3 {
                tss.privilege_stack_table[i] = create_ministack(STACK_SIZE);
            }

            tss
        })
        .collect()
});

pub static GDT: Lazy<GdtInfo> = Lazy::new(|| {
    let cpu_cnt = SMP_RESPONSE.cpus().len();

    let mut gdt = GlobalDescriptorTable::<GDT_SIZE>::empty();
    let mut tss = Vec::with_capacity(cpu_cnt);

    let kernel_code = gdt.append(Descriptor::kernel_code_segment());
    let kernel_data = gdt.append(Descriptor::kernel_data_segment());
    let user_data = gdt.append(Descriptor::user_data_segment());
    let user_code = gdt.append(Descriptor::user_code_segment());

    for i in 0..cpu_cnt {
        tss.push(gdt.append(Descriptor::tss_segment(&TSS[i])));
    }

    GdtInfo {
        gdt,
        kernel_code,
        kernel_data,
        user_code,
        user_data,
        tss,
    }
});

#[derive(Debug)]
pub struct GdtInfo {
    pub gdt: GlobalDescriptorTable<GDT_SIZE>,

    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: Vec<SegmentSelector>,
}

pub fn init(cpuid: CpuId) {
    GDT.gdt.load();

    unsafe {
        CS::set_reg(GDT.kernel_code);
        SS::set_reg(GDT.kernel_data);

        load_tss(GDT.tss[usize::from(cpuid)]);
    }
}
