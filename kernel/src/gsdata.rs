use core::{
    fmt::{Debug, Display},
    marker::PhantomPinned,
    pin::Pin,
};

use alloc::boxed::Box;
use x2apic::lapic::LocalApic;
use x86_64::{
    registers::model_specific::{GsBase, KernelGsBase},
    VirtAddr,
};

use crate::process::ThreadId;

const KERNEL_GS_MAGIC: u32 = 0x53474b21; // !KGS
const THREAD_GS_MAGIC: u32 = 0x53475421; // !TGS

#[derive(Clone, Copy, Debug)]
pub struct CpuId(u32);

impl CpuId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn find() -> Option<Self> {
        if let Ok(gsdata) = KernelGsData::load_gsbase() {
            Some(gsdata.cpuid)
        } else if let Ok(gsdata) = ThreadGsData::load_gsbase() {
            Some(gsdata.cpuid)
        } else {
            None
        }
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
    magic: u32,
    pub cpuid: CpuId,
    pub syscall_stack: VirtAddr,
    pub lapic: Box<LocalApic>,
    pub active_thread: ThreadId,
    _marker: PhantomPinned,
}

#[derive(Debug)]
#[repr(C)]
pub struct ThreadGsData {
    magic: u32,
    pub cpuid: CpuId,
    pub rsp: u64,
    _marker: PhantomPinned,
}

impl KernelGsData {
    pub fn new(
        cpuid: CpuId,
        syscall_stack: VirtAddr,
        lapic: LocalApic,
        active_thread: ThreadId,
    ) -> Self {
        Self {
            magic: KERNEL_GS_MAGIC,
            cpuid,
            syscall_stack,
            lapic: Box::new(lapic),
            active_thread,
            _marker: PhantomPinned,
        }
    }
}

impl ThreadGsData {
    pub fn new(cpuid: CpuId) -> Self {
        Self {
            magic: THREAD_GS_MAGIC,
            cpuid,
            rsp: 0,
            _marker: PhantomPinned,
        }
    }
}

impl KernelGsData {
    pub fn save_gsbase(self: Pin<&Self>) {
        GsBase::write(VirtAddr::from_ptr(&*self as *const _));
    }

    pub fn save_kernel_gsbase(self: Pin<&Self>) {
        KernelGsBase::write(VirtAddr::from_ptr(&*self as *const _));
    }

    pub fn load_gsbase() -> Result<&'static mut Self, GsDataError> {
        Self::load_address(GsBase::read())
    }

    pub fn load_kernel_gsbase() -> Result<&'static mut Self, GsDataError> {
        Self::load_address(KernelGsBase::read())
    }

    fn load_address(address: VirtAddr) -> Result<&'static mut Self, GsDataError> {
        if address < VirtAddr::new(0xFFFF_FFFF_8000_0000) {
            return Err(GsDataError::InvalidPointer(address));
        }

        let pointer = address.as_mut_ptr::<Self>();

        let data = unsafe { &mut *pointer };

        match data.magic {
            KERNEL_GS_MAGIC => Ok(data),
            THREAD_GS_MAGIC => Err(GsDataError::WrongType),
            _ => Err(GsDataError::InvalidMagic(data.magic)),
        }
    }
}

impl ThreadGsData {
    pub fn save_gsbase(self: Pin<&Self>) {
        GsBase::write(VirtAddr::from_ptr(&*self as *const _));
    }

    pub fn save_kernel_gsbase(self: Pin<&Self>) {
        KernelGsBase::write(VirtAddr::from_ptr(&*self as *const _));
    }

    pub fn load_gsbase() -> Result<&'static mut Self, GsDataError> {
        Self::load_address(GsBase::read())
    }

    pub fn load_kernel_gsbase() -> Result<&'static mut Self, GsDataError> {
        Self::load_address(KernelGsBase::read())
    }

    fn load_address(address: VirtAddr) -> Result<&'static mut Self, GsDataError> {
        if address < VirtAddr::new(0xFFFF_FFFF_8000_0000) {
            return Err(GsDataError::InvalidPointer(address));
        }

        let pointer = address.as_mut_ptr::<Self>();

        let data = unsafe { &mut *pointer };

        match data.magic {
            THREAD_GS_MAGIC => Ok(data),
            KERNEL_GS_MAGIC => Err(GsDataError::WrongType),
            _ => Err(GsDataError::InvalidMagic(data.magic)),
        }
    }
}

#[derive(Debug)]
pub enum GsDataError {
    InvalidMagic(u32),
    WrongType,
    InvalidPointer(VirtAddr),
}
