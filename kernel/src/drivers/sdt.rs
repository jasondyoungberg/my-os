use core::{fmt::Debug, slice};

use alloc::{string::String, vec::Vec};
use spin::Lazy;
use x86_64::PhysAddr;

use crate::{mapping::physical_to_virtual, RSDP_RESPONSE};

pub static ROOT_SDT: Lazy<RootSdt> = Lazy::new(|| {
    let rsdp_ptr = RSDP_RESPONSE.address();
    RootSdt::new(rsdp_ptr as *const Rsdp)
});

#[derive(Debug)]
pub struct RootSdt {
    pub header: &'static AcpiSdtHeader,
    pub entries: Vec<SystemDescriptorTable>,
}

impl RootSdt {
    pub fn new(ptr: *const Rsdp) -> Self {
        let rsdp = unsafe { &*ptr };
        let sum = unsafe { slice::from_raw_parts(ptr as *const u8, 20) }
            .iter()
            .fold(0u8, |sum, &b| sum.wrapping_add(b));
        assert!(sum == 0, "RSDP checksum error");
        assert!(rsdp.signature == *b"RSD PTR ", "RSDP signature mismatch");

        let (header, entry_addr) = match rsdp.revision {
            0 => {
                let rsdt_ptr =
                    physical_to_virtual(PhysAddr::new(rsdp.rsdt_address as u64)).as_ptr();
                let rsdt = AcpiSdtHeader::new(rsdt_ptr).unwrap();
                let rsdt_entry_cnt = (rsdt.length - 36) / 8;
                let rsdt_entries = unsafe { rsdt_ptr.byte_offset(36) } as *const [u8; 4];

                (
                    rsdt,
                    (0..rsdt_entry_cnt)
                        .map(|i| {
                            u32::from_le_bytes(unsafe { *rsdt_entries.offset(i as isize) }) as u64
                        })
                        .collect::<Vec<_>>(),
                )
            }
            2 => {
                let sum = unsafe { slice::from_raw_parts(ptr as *const u8, 36) }
                    .iter()
                    .fold(0u8, |sum, &b| sum.wrapping_add(b));
                assert!(sum == 0, "RSDP checksum error");

                let xsdt_ptr = physical_to_virtual(PhysAddr::new(rsdp.xsdt_address)).as_ptr();
                let xsdt = AcpiSdtHeader::new(xsdt_ptr).unwrap();
                let xsdt_entry_cnt = (xsdt.length - 36) / 8;
                let xsdt_entries = unsafe { xsdt_ptr.byte_offset(36) } as *const [u8; 8];

                (
                    xsdt,
                    (0..xsdt_entry_cnt)
                        .map(|i| u64::from_le_bytes(unsafe { *xsdt_entries.offset(i as isize) }))
                        .collect(),
                )
            }
            _ => panic!("Unknown ACPI revision"),
        };

        let entries = entry_addr
            .iter()
            .map(|&addr| PhysAddr::new(addr))
            .map(physical_to_virtual)
            .map(|virt| virt.as_ptr())
            .map(|ptr| AcpiSdtHeader::new(ptr).unwrap())
            .map(|header| match &header.signature {
                b"APIC" => SystemDescriptorTable::Madt(MultipleApicDescriptionTable { header }),
                _ => SystemDescriptorTable::Unknown(header),
            })
            .collect();

        Self { header, entries }
    }
}

#[derive(Debug)]
pub enum SystemDescriptorTable {
    Madt(MultipleApicDescriptionTable),
    Unknown(&'static AcpiSdtHeader),
}

#[derive(Debug)]
pub struct MultipleApicDescriptionTable {
    pub header: &'static AcpiSdtHeader,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Rsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,

    // version 2
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    _reserved: [u8; 3],
}

#[repr(C)]
pub struct AcpiSdtHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}
impl AcpiSdtHeader {
    fn new(ptr: *const Self) -> Option<&'static Self> {
        let header = unsafe { &*ptr };
        let bytes = unsafe { slice::from_raw_parts(ptr as *const u8, header.length as usize) };
        let checksum = bytes.iter().fold(0u8, |sum, &b| sum.wrapping_add(b));
        if checksum == 0 {
            Some(header)
        } else {
            None
        }
    }
}
impl Debug for AcpiSdtHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AcpiSdtHeader")
            .field("signature", &String::from_utf8_lossy(&self.signature))
            .field("length", &self.length)
            .field("revision", &self.revision)
            .field("checksum", &self.checksum)
            .field("oem_id", &String::from_utf8_lossy(&self.oem_id))
            .field("oem_table_id", &String::from_utf8_lossy(&self.oem_table_id))
            .field("oem_revision", &self.oem_revision)
            .field("creator_id", &self.creator_id)
            .field("creator_revision", &self.creator_revision)
            .finish()
    }
}
