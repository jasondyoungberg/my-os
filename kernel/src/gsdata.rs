use alloc::boxed::Box;

use crate::{
    address::VirtAddr,
    drivers::lapic::{self, LocalApic},
    registers::{Gs, KernelGsBase},
};

const MAGIC: [u8; 8] = *b"GS DATA ";

#[derive(Debug)]
#[repr(C)]
pub struct GsData {
    self_ptr: VirtAddr,
    magic: [u8; 8],
    syscall_rsp: VirtAddr,
    sysret_rsp: VirtAddr,
    pub cpu_id: u32,
    pub lapic: LocalApic,
}

impl GsData {
    pub fn init(syscall_rsp: VirtAddr, cpu_id: u32, lapic: LocalApic) {
        let gsdata = Self {
            self_ptr: VirtAddr::null(),
            magic: MAGIC,
            syscall_rsp,
            sysret_rsp: VirtAddr::null(),
            cpu_id,
            lapic: LocalApic::new(),
        };

        let ptr = Box::into_raw(Box::new(gsdata));
        let addr = VirtAddr::from_ptr(ptr);
        let data = unsafe { &mut *ptr };
        data.self_ptr = addr;

        KernelGsBase::write(addr);
    }

    pub fn load() -> Option<&'static mut Self> {
        let addr = VirtAddr::try_new(Gs::read_at())?;
        let data = unsafe { &mut *(addr.as_mut_ptr::<Self>()) };

        if data.self_ptr == addr && data.magic == MAGIC {
            Some(data)
        } else {
            None
        }
    }
}
