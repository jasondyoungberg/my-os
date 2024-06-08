use core::ptr::NonNull;

use acpi::InterruptModel;
use spin::{Lazy, Mutex};
use volatile::{access::ReadWrite, VolatileRef};
use x86_64::{
    structures::paging::{PageTableFlags, PhysFrame},
    PhysAddr,
};

use crate::{allocation::page::MMIO_ALLOCATOR, mapping::map_kernel_page_to_frame};

use super::acpi::acpi_tables;

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
        todo!()
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
