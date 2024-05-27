use core::{
    cell::UnsafeCell,
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

#[derive(Clone, Copy, Debug)]
pub struct CpuId(u32);

impl CpuId {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn find() -> Option<Self> {
        if let Some(gsdata) = KernelData::load_gsbase() {
            Some(gsdata.cpuid)
        } else if let Some(gsdata) = KernelData::load_kernel_gsbase() {
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
pub struct KernelData {
    self_address: UnsafeCell<VirtAddr>,
    pub sysret_stack: VirtAddr,
    pub syscall_stack: VirtAddr, // don't move this field without updating the offset in syscall wrapper
    pub cpuid: CpuId,
    pub lapic: Box<LocalApic>,
    pub active_thread: ThreadId,
    _marker: PhantomPinned,
}

impl KernelData {
    pub fn new(
        cpuid: CpuId,
        syscall_stack: VirtAddr,
        lapic: LocalApic,
        active_thread: ThreadId,
    ) -> Pin<Box<Self>> {
        let ret = Box::pin(Self {
            self_address: UnsafeCell::new(VirtAddr::zero()),
            sysret_stack: VirtAddr::zero(),
            syscall_stack,
            cpuid,
            lapic: Box::new(lapic),
            active_thread,
            _marker: PhantomPinned,
        });
        let address = VirtAddr::from_ptr(&*ret as *const _);
        unsafe {
            *ret.self_address.get() = address;
        }
        ret
    }
}

impl KernelData {
    pub fn save_gsbase(self: Pin<&Self>) {
        GsBase::write(VirtAddr::from_ptr(&*self as *const _));
    }

    pub fn save_kernel_gsbase(self: Pin<&Self>) {
        KernelGsBase::write(VirtAddr::from_ptr(&*self as *const _));
    }

    pub fn load_gsbase() -> Option<&'static mut Self> {
        Self::load_address(GsBase::read())
    }

    pub fn load_kernel_gsbase() -> Option<&'static mut Self> {
        Self::load_address(KernelGsBase::read())
    }

    fn load_address(address: VirtAddr) -> Option<&'static mut Self> {
        if address < VirtAddr::new(0xFFFF_FFFF_8000_0000) {
            return None;
        }

        let pointer = address.as_mut_ptr::<Self>();

        let data = unsafe { &mut *pointer };

        if unsafe { *data.self_address.get() } != address {
            return None;
        }

        Some(data)
    }
}

#[derive(Debug)]
pub enum GsDataError {
    InvalidMagic(u32),
    WrongType,
    InvalidPointer(VirtAddr),
}
