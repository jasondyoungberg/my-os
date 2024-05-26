use core::fmt::Display;

use alloc::boxed::Box;
use x2apic::lapic::LocalApic;
use x86_64::registers::model_specific::GsBase;

use crate::process::ThreadId;

#[derive(Clone, Copy, Debug)]
pub struct CpuId(u32);

impl CpuId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
}

impl Display for CpuId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CPU{}", self.0)
    }
}

impl From<CpuId> for usize {
    fn from(cpu_id: CpuId) -> usize {
        cpu_id.0 as usize
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct KernelGsData {
    pub syscall_stack: u64,
    pub cpuid: CpuId,
    pub lapic: Box<LocalApic>,
    pub active_thread: ThreadId,
}

#[derive(Debug)]
#[repr(C)]
pub struct UserGsData {
    pub rsp: u64,
    pub cpuid: CpuId,
}

pub fn get_kernel_gs_data() -> Option<&'static mut KernelGsData> {
    let core_data_addr = GsBase::read();
    if core_data_addr >= x86_64::VirtAddr::new(0xFFFF_FFFF_8000_0000) {
        let core_data_ptr = core_data_addr.as_u64() as *mut KernelGsData;
        Some(unsafe { &mut *core_data_ptr })
    } else {
        None
    }
}
