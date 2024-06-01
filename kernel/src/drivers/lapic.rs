use crate::{
    allocation::page::MMIO_ALLOCATOR,
    mapping::map_kernel_page_to_frame,
    registers::ApicBase,
    structures::{mmio::Mmio, paging::PageTableFlags},
};

const EOI: usize = 0xb0;
const SPURIOUS_VECTOR: usize = 0xf0;
const LVT_TIMER: usize = 0x320;
const INITIAL_COUNT: usize = 0x380;
const DIVIDE_CONFIG: usize = 0x3e0;

#[derive(Debug)]
pub struct LocalApic(Mmio);
impl LocalApic {
    pub fn new() -> Self {
        let frame = ApicBase::get_base();
        let page = MMIO_ALLOCATOR.alloc(1);
        map_kernel_page_to_frame(
            page,
            frame,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE,
        );
        let mmio = unsafe { Mmio::new(page.start(), 0x400) };

        Self(mmio)
    }

    pub fn init(&self) {
        ApicBase::enable();
        let mut svr = self.0.read::<u32>(SPURIOUS_VECTOR);
        svr |= 1 << 8;
        self.0.write::<u32>(SPURIOUS_VECTOR, svr);

        self.0.write::<u32>(LVT_TIMER, 0x0002_0020);
        self.0.write::<u32>(DIVIDE_CONFIG, 0b1011);
        self.0.write::<u32>(INITIAL_COUNT, 1_000_000_000);
    }

    pub fn eoi(&self) {
        self.0.write::<u32>(EOI, 0);
    }
}
