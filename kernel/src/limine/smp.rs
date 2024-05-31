use core::sync::atomic::{AtomicPtr, Ordering};

use bitflags::bitflags;

use super::{Response, MAGIC_1, MAGIC_2};

#[repr(C)]
pub struct SmpRequest {
    id: [u64; 4],
    revision: u64,
    pub response: Response<SmpResponse>,
    flags: SmpFlags,
}
impl SmpRequest {
    pub const fn new(flags: SmpFlags) -> Self {
        Self {
            id: [MAGIC_1, MAGIC_2, 0x95a67b819a1b857e, 0xa0b61b723b6a73e0],
            revision: 0,
            response: Response::none(),
            flags,
        }
    }
}
unsafe impl Sync for SmpRequest {}
unsafe impl Send for SmpRequest {}

#[repr(C)]
#[derive(Debug)]
pub struct SmpResponse {
    revision: u64,
    flags: SmpFlags,
    bsp_lapic_id: u32,
    cpu_count: u64,
    cpus: *const *const SmpInfo,
}
impl SmpResponse {
    pub fn cpus(&self) -> &[&SmpInfo] {
        unsafe { core::slice::from_raw_parts(self.cpus.cast(), self.cpu_count as usize) }
    }
}

bitflags! {
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct SmpFlags: u32 {
        const X2APIC = 1 << 0;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SmpInfo {
    pub processor_id: u32,
    pub lapic_id: u32,
    _reserved: u64,
    pub goto_address: GotoAddress,
    pub extra: u64,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct GotoAddress(AtomicPtr<()>);
impl GotoAddress {
    pub fn write(&self, func: unsafe extern "C" fn(&SmpInfo) -> !) {
        self.0.store(func as *mut (), Ordering::Release);
    }
}
