use bitflags::bitflags;

use crate::{
    address::VirtAddr,
    dbg,
    mapping::map_kernel_page_to_frame,
    println,
    registers::ApicBase,
    structures::{
        self,
        mmio::{self, Mmio},
        paging::PageTableFlags,
    },
};

const EOI: usize = 0xb0;
const LVT_TIMER: usize = 0x320;
const INITIAL_COUNT: usize = 0x380;
const DIVIDE_CONFIG: usize = 0x3e0;

#[derive(Debug)]
pub struct LocalApic(Mmio);
impl LocalApic {
    pub fn new() -> Self {
        let frame = ApicBase::get_base();
        let page = structures::paging::Page::containing_addr(VirtAddr::new(0xffff_e000_0000_0000));
        map_kernel_page_to_frame(
            page,
            frame,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE,
        );
        let mmio = unsafe { Mmio::new(page.start(), 0x400) };
        Self(mmio)
    }

    pub fn init(&self) {
        self.0.write::<u32>(LVT_TIMER, 0x0002_0020);
        self.0.write::<u32>(DIVIDE_CONFIG, 0b1011);
        self.0.write::<u32>(INITIAL_COUNT, 1_000_000_000);
    }

    pub fn eoi(&self) {
        self.0.write::<u32>(EOI, 0);
    }
}
