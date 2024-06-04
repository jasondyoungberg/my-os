use core::ptr::NonNull;

use bit_field::BitField;
use volatile::{
    access::{ReadOnly, ReadWrite, WriteOnly},
    VolatilePtr,
};
use x86_64::{
    registers::model_specific::Msr,
    structures::paging::{PageTableFlags, PhysFrame},
    PhysAddr,
};

use crate::{allocation::page::MMIO_ALLOCATOR, mapping::map_kernel_page_to_frame};

const EOI: usize = 0xb0;
const SPURIOUS_VECTOR: usize = 0xf0;
const LVT_TIMER: usize = 0x320;
const INITIAL_COUNT: usize = 0x380;
const DIVIDE_CONFIG: usize = 0x3e0;

#[derive(Debug)]
pub struct LocalApic<'a> {
    apic_base_msr: Msr,
    local_apic_id: VolatilePtr<'a, u32, ReadWrite>,
    local_apic_version: VolatilePtr<'a, u32, ReadOnly>,
    // ...
    eoi: VolatilePtr<'a, u32, WriteOnly>,
    // ...
    spurios_interrupt_vector: VolatilePtr<'a, u32, ReadWrite>,
    // ...
    lvt_timer: VolatilePtr<'a, u32, ReadWrite>,
    // ...
    initial_count: VolatilePtr<'a, u32, ReadWrite>,
    current_count: VolatilePtr<'a, u32, ReadOnly>,
    divide_configuration: VolatilePtr<'a, u32, ReadWrite>,
}
impl LocalApic<'_> {
    pub fn new() -> Self {
        let apic_base_msr = Msr::new(0x1b);

        let frame = PhysFrame::containing_address(PhysAddr::new(unsafe { apic_base_msr.read() }));

        let page = MMIO_ALLOCATOR.alloc();
        unsafe {
            map_kernel_page_to_frame(
                page,
                frame,
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE,
            )
        };

        let ptr = page.start_address().as_mut_ptr::<u32>();
        let base = NonNull::new(ptr).expect("Failed to create NonNull");
        unsafe {
            let local_apic_id = VolatilePtr::new(base.byte_add(0x20));
            let local_apic_version = VolatilePtr::new(base.byte_add(0x30)).read_only();
            let eoi = VolatilePtr::new(base.byte_add(0xb0)).write_only();
            let spurios_interrupt_vector = VolatilePtr::new(base.byte_add(0xf0));
            let lvt_timer = VolatilePtr::new(base.byte_add(0x320));
            let initial_count = VolatilePtr::new(base.byte_add(0x380));
            let current_count = VolatilePtr::new(base.byte_add(0x390)).read_only();
            let divide_configuration = VolatilePtr::new(base.byte_add(0x3e0));

            Self {
                apic_base_msr,
                local_apic_id,
                local_apic_version,
                eoi,
                spurios_interrupt_vector,
                lvt_timer,
                initial_count,
                current_count,
                divide_configuration,
            }
        }
    }

    pub fn init(&mut self) {
        {
            let mut x = unsafe { self.apic_base_msr.read() };
            x.set_bit(11, true);
            unsafe { self.apic_base_msr.write(x) };
        }

        {
            let mut x = self.spurios_interrupt_vector.read();
            x.set_bit(8, true);
            self.spurios_interrupt_vector.write(x);
        }

        // self.0.write::<u32>(LVT_TIMER, 0x0002_0020);
        self.lvt_timer.write(0x0002_0020);
        self.divide_configuration.write(0b1011);
        self.initial_count.write(100_000_000);
        // self.initial_count.write(25_000_000);
    }

    pub fn signal_eoi(&self) {
        self.eoi.write(0);
    }
}
