use alloc::boxed::Box;
use x86_64::{
    registers::model_specific::{GsBase, KernelGsBase},
    VirtAddr,
};

use crate::drivers::lapic::LocalApic;

const MAGIC: [u8; 8] = *b"GS DATA ";

#[derive(Debug)]
#[repr(C)]
pub struct GsData {
    self_ptr: VirtAddr,
    magic: [u8; 8],
    syscall_rsp: VirtAddr,
    sysret_rsp: VirtAddr,
    pub cpuid: u64,
    pub lapic: LocalApic<'static>,
}

impl GsData {
    pub fn init(syscall_rsp: VirtAddr, cpuid: usize, lapic: LocalApic<'static>) {
        let gsdata = Self {
            self_ptr: VirtAddr::zero(),
            magic: MAGIC,
            syscall_rsp,
            sysret_rsp: VirtAddr::zero(),
            cpuid: cpuid as u64,
            lapic,
        };

        let ptr = Box::into_raw(Box::new(gsdata));
        let addr = VirtAddr::from_ptr(ptr);
        let data = unsafe { &mut *ptr };
        data.self_ptr = addr;

        KernelGsBase::write(addr);
    }

    pub fn load() -> Option<&'static mut Self> {
        let addr = GsBase::read();
        let data = unsafe { &mut *(addr.as_mut_ptr::<Self>()) };

        if data.self_ptr == addr && data.magic == MAGIC {
            Some(data)
        } else {
            None
        }
    }
}
