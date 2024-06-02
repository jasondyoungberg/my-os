use alloc::vec::Vec;
use spin::Lazy;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        paging::PageTableFlags,
        tss::TaskStateSegment,
    },
    VirtAddr,
};

use crate::{allocation::page::STACK_ALLOCATOR, mapping::map_kernel_page, SMP_RESPONSE};

const STACK_SIZE: u64 = 64 * 1024;

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
    (0..cpu_count)
        .map(|_| {
            let mut tss = TaskStateSegment::new();

            for i in 0..7 {
                tss.interrupt_stack_table[i] = create_stack();
            }

            for i in 0..3 {
                tss.privilege_stack_table[i] = create_stack();
            }

            tss
        })
        .collect()
});

struct GdtInfo {
    gdt: GlobalDescriptorTable<69>,
    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
    user_data: SegmentSelector,
    user_code: SegmentSelector,
    tss: Vec<SegmentSelector>,
}

fn create_stack() -> VirtAddr {
    let pages = STACK_ALLOCATOR.alloc_range(STACK_SIZE);
    let rsp = pages.end.start_address();
    for page in pages {
        unsafe { map_kernel_page(page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE) };
    }
    rsp
}

pub fn init(cpu_id: u32) {
    GDT.gdt.load();

    unsafe {
        CS::set_reg(GDT.kernel_code);
        SS::set_reg(GDT.kernel_data);

        load_tss(GDT.tss[cpu_id as usize])
    }
}
