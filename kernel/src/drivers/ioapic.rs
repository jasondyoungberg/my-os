use core::ptr::NonNull;

use acpi::{
    platform::interrupt::{Polarity, TriggerMode},
    InterruptModel,
};
use bitflags::bitflags;
use spin::{Lazy, Mutex};
use volatile::{access::ReadWrite, VolatileRef};
use x86_64::{
    structures::paging::{PageTableFlags, PhysFrame},
    PhysAddr,
};

use crate::{allocation::page::MMIO_ALLOCATOR, mapping::map_kernel_page_to_frame};

use super::acpi::acpi_tables;

pub const PIT_VECTOR: u8 = 0x40;
pub const KEYBOARD_VECTOR: u8 = 0x41;
pub const CASCADE_VECTOR: u8 = 0x42;
pub const COM2_VECTOR: u8 = 0x43;
pub const COM1_VECTOR: u8 = 0x44;
pub const LPT2_VECTOR: u8 = 0x45;
pub const FLOPPY_VECTOR: u8 = 0x46;
pub const LPT1_VECTOR: u8 = 0x47;
pub const RTC_VECTOR: u8 = 0x48;
pub const ACPI_VECTOR: u8 = 0x49;
pub const MOUSE_VECTOR: u8 = 0x4C;
pub const COPROCESSOR_VECTOR: u8 = 0x4D;
pub const PRIMARY_ATA_VECTOR: u8 = 0x4E;
pub const SECONDARY_ATA_VECTOR: u8 = 0x4F;

pub const IOAPIC_RANGE_START: u8 = 0x40;
pub const IOAPIC_RANGE_END: u8 = 0xFF;

pub static IOAPIC: Lazy<Mutex<Ioapic>> = Lazy::new(|| Mutex::new(Ioapic::new()));

pub struct Ioapic {
    ioregsel: VolatileRef<'static, u32, ReadWrite>,
    iowin: VolatileRef<'static, u32, ReadWrite>,
}

impl Ioapic {
    fn new() -> Self {
        let tables = acpi_tables();
        let interrupt_model = tables
            .platform_info()
            .expect("Failed to get platform info")
            .interrupt_model;

        let apic = match interrupt_model {
            InterruptModel::Unknown => panic!("Unknown interrupt model"),
            InterruptModel::Apic(apic) => apic,
            _ => panic!("Unknown interrupt model"),
        };

        assert_eq!(apic.io_apics.len(), 1, "Only one IOAPIC is supported");

        let ioapic_data = &apic.io_apics[0];
        let phys = PhysAddr::new(ioapic_data.address as u64);

        let frame = PhysFrame::containing_address(phys);
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
            Self {
                ioregsel: VolatileRef::new(base),
                iowin: VolatileRef::new(base.byte_add(0x10)),
            }
        }
    }

    pub fn init(&mut self) {
        log::info!(
            "IOAPIC: {:#x} {:#x} {:#x}",
            self.read(0),
            self.read(1),
            self.read(2)
        );

        let entries = ((self.read(1) >> 16) & 0xFF) + 1;
        assert!(
            entries <= (IOAPIC_RANGE_END - IOAPIC_RANGE_START + 1) as u32,
            "IOAPIC: Too many entries"
        );

        let tables = acpi_tables();
        let interrupt_model = tables
            .platform_info()
            .expect("Failed to get platform info")
            .interrupt_model;

        let apic = match interrupt_model {
            InterruptModel::Unknown => panic!("Unknown interrupt model"),
            InterruptModel::Apic(apic) => apic,
            _ => panic!("Unknown interrupt model"),
        };

        for i in 0..entries {
            self.map(
                i as u8,
                IOAPIC_RANGE_START + i as u8,
                DeliveryMode::Fixed,
                IoapicFlags::empty(),
                0,
            );
        }

        for int_override in apic.interrupt_source_overrides.iter() {
            let flags = match int_override.polarity {
                Polarity::SameAsBus => IoapicFlags::empty(),
                Polarity::ActiveHigh => IoapicFlags::empty(),
                Polarity::ActiveLow => IoapicFlags::ACTIVE_LOW,
            } | match int_override.trigger_mode {
                TriggerMode::SameAsBus => IoapicFlags::empty(),
                TriggerMode::Edge => IoapicFlags::empty(),
                TriggerMode::Level => IoapicFlags::LEVEL_TRIGGERED,
            };

            self.map(
                int_override.global_system_interrupt as u8,
                int_override.isa_source + IOAPIC_RANGE_START,
                DeliveryMode::Fixed,
                flags,
                0,
            );
        }
    }

    fn map(
        &mut self,
        irq: u8,
        vector: u8,
        delivery_mode: DeliveryMode,
        flags: IoapicFlags,
        destination: u8,
    ) {
        let reg = 0x10 + irq * 2;
        let data =
            (vector as u64) | ((destination as u64) << 56) | (delivery_mode as u64) | flags.bits();
        self.write(reg, data as u32);
        self.write(reg + 1, (data >> 32) as u32);
    }

    fn read(&mut self, reg: u8) -> u32 {
        self.ioregsel.as_mut_ptr().write(reg as u32);
        self.iowin.as_ptr().read()
    }

    fn write(&mut self, reg: u8, data: u32) {
        self.ioregsel.as_mut_ptr().write(reg as u32);
        self.iowin.as_mut_ptr().write(data);
    }
}

#[repr(u8)]
enum DeliveryMode {
    Fixed = 0b000,
    LowestPriority = 0b001,
    Smi = 0b010,
    Nmi = 0b100,
    Init = 0b101,
    ExtINT = 0b111,
}

bitflags! {
    pub struct IoapicFlags: u64 {
        const LOGICAL = 1 << 11;
        const SEND_PENDING = 1 << 12;
        const ACTIVE_LOW = 1 << 13;
        const REMOTE_IRR = 1 << 14;
        const LEVEL_TRIGGERED = 1 << 15;
        const MASKED = 1 << 16;
    }
}
