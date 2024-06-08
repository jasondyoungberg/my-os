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

pub const LAPIC_RANGE_START: u8 = 0x30;
pub const LAPIC_RANGE_END: u8 = 0x3E;

pub const CMCI_VECTOR: u8 = 0x30;
pub const TIMER_VECTOR: u8 = 0x31;
pub const THERMAL_VECTOR: u8 = 0x32;
pub const PERFORMANCE_VECTOR: u8 = 0x33;
pub const LINT0_VECTOR: u8 = 0x34;
pub const LINT1_VECTOR: u8 = 0x35;
pub const ERROR_VECTOR: u8 = 0x36;
pub const SPURIOS_VECTOR: u8 = 0x3F;

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
    lvt_cmci: VolatilePtr<'a, u32, ReadWrite>,
    icr_low: VolatilePtr<'a, u32, ReadWrite>,
    icr_high: VolatilePtr<'a, u32, ReadWrite>,
    lvt_timer: VolatilePtr<'a, u32, ReadWrite>,
    lvt_thermal_sensor: VolatilePtr<'a, u32, ReadWrite>,
    lvt_performance: VolatilePtr<'a, u32, ReadWrite>,
    lvt_lint0: VolatilePtr<'a, u32, ReadWrite>,
    lvt_lint1: VolatilePtr<'a, u32, ReadWrite>,
    lvt_error: VolatilePtr<'a, u32, ReadWrite>,
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
            let lvt_cmci = VolatilePtr::new(base.byte_add(0x2f0));
            let icr_low = VolatilePtr::new(base.byte_add(0x300));
            let icr_high = VolatilePtr::new(base.byte_add(0x310));
            let lvt_timer = VolatilePtr::new(base.byte_add(0x320));
            let lvt_thermal_sensor = VolatilePtr::new(base.byte_add(0x330));
            let lvt_performance = VolatilePtr::new(base.byte_add(0x340));
            let lvt_lint0 = VolatilePtr::new(base.byte_add(0x350));
            let lvt_lint1 = VolatilePtr::new(base.byte_add(0x360));
            let lvt_error = VolatilePtr::new(base.byte_add(0x370));
            let initial_count = VolatilePtr::new(base.byte_add(0x380));
            let current_count = VolatilePtr::new(base.byte_add(0x390)).read_only();
            let divide_configuration = VolatilePtr::new(base.byte_add(0x3e0));

            Self {
                apic_base_msr,
                local_apic_id,
                local_apic_version,
                eoi,
                spurios_interrupt_vector,
                lvt_cmci,
                icr_low,
                icr_high,
                lvt_timer,
                lvt_thermal_sensor,
                lvt_performance,
                lvt_lint0,
                lvt_lint1,
                lvt_error,
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

        self.spurios_interrupt_vector
            .write(0x0000_0100 | SPURIOS_VECTOR as u32); // apic enable

        self.lvt_timer.write(0x0002_0000 | TIMER_VECTOR as u32); // periodic
        self.lvt_cmci.write(CMCI_VECTOR as u32);
        self.lvt_lint0.write(LINT0_VECTOR as u32);
        self.lvt_lint1.write(LINT1_VECTOR as u32);
        self.lvt_error.write(ERROR_VECTOR as u32);
        self.lvt_performance.write(PERFORMANCE_VECTOR as u32);
        self.lvt_thermal_sensor.write(THERMAL_VECTOR as u32);
        self.divide_configuration.write(0b1011);
        self.initial_count.write(100_000_000);
    }

    pub fn send_ipi(&mut self, vector: u8) {
        let mut data: u64 = 0;
        data.set_bits(0..=7, vector as u64);
        data.set_bits(8..=10, 0b000); // fixed
        data.set_bit(11, false); // dest: physical
        data.set_bit(12, false); // delivery status: idle
        data.set_bit(14, false); // level: deassert
        data.set_bit(15, false); // trigger mode: edge
        data.set_bits(18..=19, 0b11); // dest mode: all excluding self

        log::info!("ipi: {:#x}", data);

        self.icr_high.write(data.get_bits(32..=63) as u32);
        self.icr_low.write(data as u32);
    }

    pub fn signal_eoi(&self) {
        self.eoi.write(0);
    }
}
