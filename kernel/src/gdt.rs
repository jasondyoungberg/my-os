use alloc::vec::Vec;
use spin::Lazy;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

use crate::{gsdata::CpuId, SMP_RESPONSE};

const MAX_CORES: usize = 4;
const GDT_SIZE: usize = 6 + 2 * MAX_CORES;
const STACK_SIZE: usize = 65536;

static STACKS: Lazy<Vec<Stack>> = Lazy::new(|| {
    (0..(SMP_RESPONSE.cpus().len() * 10))
        .map(|_| Stack([0; STACK_SIZE]))
        .collect()
});

pub static TSS: Lazy<Vec<TaskStateSegment>> = Lazy::new(|| {
    let cpu_cnt = SMP_RESPONSE.cpus().len();
    (0..cpu_cnt)
        .map(|tss_idx| {
            let mut tss = TaskStateSegment::new();

            for ist_idx in 0..7 {
                let stack_idx = tss_idx * 10 + ist_idx;
                tss.interrupt_stack_table[ist_idx] = STACKS[stack_idx].addr();
            }

            for pst_idx in 0..3 {
                let stack_idx = tss_idx * 10 + 7 + pst_idx;
                tss.privilege_stack_table[pst_idx] = STACKS[stack_idx].addr();
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

#[repr(C, align(16))]
struct Stack([u8; STACK_SIZE]);

impl Stack {
    pub fn addr(&self) -> VirtAddr {
        VirtAddr::from_ptr(self.0.as_ptr()) + STACK_SIZE as u64
    }
}
