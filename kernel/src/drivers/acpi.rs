use core::ptr::NonNull;

use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use x86_64::{PhysAddr, VirtAddr};

use crate::{
    mapping::{hhdm, hhdm_reverse},
    RSDP_RESPONSE,
};

pub fn acpi_tables() -> AcpiTables<Handler> {
    unsafe {
        AcpiTables::from_rsdp(
            Handler,
            hhdm_reverse(VirtAddr::new(RSDP_RESPONSE.address() as u64)).as_u64() as usize,
        )
        .expect("Failed to parse ACPI tables")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Handler;

impl AcpiHandler for Handler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        unsafe {
            PhysicalMapping::new(
                physical_address,
                NonNull::new(hhdm(PhysAddr::new(physical_address as u64)).as_mut_ptr::<T>())
                    .unwrap(),
                size,
                size,
                Self,
            )
        }
    }

    fn unmap_physical_region<T>(_: &acpi::PhysicalMapping<Self, T>) {}
}
