use alloc::boxed::Box;
use x86_64::{
    registers::model_specific::{GsBase, KernelGsBase},
    VirtAddr,
};

use crate::{drivers::lapic::LocalApic, process::Process};

const MAGIC: [u8; 8] = *b"GS DATA ";

#[derive(Debug)]
#[repr(C)]
pub struct GsData {
    self_ptr: VirtAddr,
    magic: [u8; 8],
    syscall_rsp: VirtAddr,
    pub sysret_rsp: VirtAddr,
    pub cpuid: usize,
    pub process: Option<Process>,
    pub lapic: LocalApic<'static>,
}

impl GsData {
    pub fn init(syscall_rsp: VirtAddr, cpuid: usize, lapic: LocalApic<'static>) {
        let gsdata = Self {
            self_ptr: VirtAddr::zero(),
            magic: MAGIC,
            syscall_rsp,
            sysret_rsp: VirtAddr::zero(),
            cpuid,
            process: None,
            lapic,
        };

        let ptr = Box::into_raw(Box::new(gsdata));
        let addr = VirtAddr::from_ptr(ptr);
        let data = unsafe { &mut *ptr };
        data.self_ptr = addr;

        KernelGsBase::write(addr);
    }

    /// # Safety
    /// This function is unsafe because it can create multiple mutable references to the same data.
    /// Make sure to drop the returned reference before calling this function again.
    pub unsafe fn load<'a>() -> Option<&'a mut Self> {
        let addr = GsBase::read();

        if addr.as_u64() < 0xFFFF_FFFF_8000_0000 {
            return None;
        }

        let data = unsafe { &mut *(addr.as_mut_ptr::<Self>()) };

        if data.self_ptr == addr && data.magic == MAGIC {
            Some(data)
        } else {
            None
        }
    }

    /// # Safety
    /// This function is unsafe because it can create multiple mutable references to the same data.
    /// Make sure to drop the returned reference before calling this function again.
    pub unsafe fn load_kernel<'a>() -> Option<&'a mut Self> {
        let addr = KernelGsBase::read();

        if addr.as_u64() < 0xFFFF_FFFF_8000_0000 {
            return None;
        }

        let data = unsafe { &mut *(addr.as_mut_ptr::<Self>()) };

        if data.self_ptr == addr && data.magic == MAGIC {
            Some(data)
        } else {
            None
        }
    }
}
