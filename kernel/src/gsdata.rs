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

    pub fn sysret_rsp() -> Result<VirtAddr, GsDataError> {
        Ok(unsafe { Self::load() }?.sysret_rsp)
    }

    pub fn cpuid() -> Result<usize, GsDataError> {
        Ok(unsafe { Self::load() }?.cpuid)
    }

    pub unsafe fn process() -> Result<Option<&'static mut Process>, GsDataError> {
        Ok(unsafe { Self::load() }?.process.as_mut())
    }

    pub fn process_take() -> Result<Option<Process>, GsDataError> {
        Ok(unsafe { Self::load() }?.process.take())
    }

    pub fn process_replace(process: Process) -> Result<Option<Process>, GsDataError> {
        Ok(unsafe { Self::load() }?.process.replace(process))
    }

    pub unsafe fn lapic() -> Result<&'static mut LocalApic<'static>, GsDataError> {
        Ok(&mut unsafe { Self::load() }?.lapic)
    }

    /// # Safety
    /// This function is unsafe because it can create multiple mutable references to the same data.
    /// Make sure to drop the returned reference before calling this function again.
    pub unsafe fn load<'a>() -> Result<&'a mut Self, GsDataError> {
        Self::from_virtual_address(GsBase::read())
            .or_else(|_| Self::from_virtual_address(KernelGsBase::read()))
    }

    fn from_virtual_address<'a>(addr: VirtAddr) -> Result<&'a mut Self, GsDataError> {
        if addr.as_u64() < 0xFFFF_FFFF_8000_0000 {
            return Err(GsDataError::AddressOutOfBounds);
        }

        let data = unsafe { &mut *(addr.as_mut_ptr::<Self>()) };

        if data.self_ptr != addr {
            return Err(GsDataError::InvalidSelfPtr);
        }

        if data.magic != MAGIC {
            return Err(GsDataError::InvalidMagic);
        }

        Ok(data)
    }
}

#[derive(Debug)]
pub enum GsDataError {
    AddressOutOfBounds,
    InvalidSelfPtr,
    InvalidMagic,
}
