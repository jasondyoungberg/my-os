use core::arch::asm;

use crate::address::VirtAddr;

use super::ModelSpecificRegister;

pub struct FsBase;
impl FsBase {
    const MSR_ADDR: u32 = 0xC000_0100;

    pub fn read() -> VirtAddr {
        VirtAddr::new(ModelSpecificRegister::<{ Self::MSR_ADDR }>::read())
    }

    pub fn write(value: VirtAddr) {
        ModelSpecificRegister::<{ Self::MSR_ADDR }>::write(value.as_u64())
    }
}

pub struct GsBase;
impl GsBase {
    const MSR_ADDR: u32 = 0xC000_0101;

    pub fn read() -> VirtAddr {
        VirtAddr::new(ModelSpecificRegister::<{ Self::MSR_ADDR }>::read())
    }

    pub fn write(value: VirtAddr) {
        ModelSpecificRegister::<{ Self::MSR_ADDR }>::write(value.as_u64())
    }
}

pub struct KernelGsBase;
impl KernelGsBase {
    const MSR_ADDR: u32 = 0xC000_0102;

    pub fn read() -> VirtAddr {
        VirtAddr::new(ModelSpecificRegister::<{ Self::MSR_ADDR }>::read())
    }

    pub fn write(value: VirtAddr) {
        ModelSpecificRegister::<{ Self::MSR_ADDR }>::write(value.as_u64())
    }
}

pub struct Fs;
impl Fs {
    pub fn read_at() -> u64 {
        let value: u64;
        unsafe { asm!("mov {}, fs:0", out(reg) value, options(nomem, preserves_flags)) };
        value
    }

    pub unsafe fn write_at(value: u64) {
        unsafe { asm!("mov fs:0, {}", in(reg) value, options(nomem, preserves_flags)) };
    }
}

pub struct Gs;
impl Gs {
    pub fn read_at() -> u64 {
        let value: u64;
        unsafe { asm!("mov {}, gs:0", out(reg) value, options(nomem, preserves_flags)) };
        value
    }

    pub unsafe fn write_at(value: u64) {
        unsafe { asm!("mov gs:0, {}", in(reg) value, options(nomem, preserves_flags)) };
    }

    pub fn swap() {
        unsafe { asm!("swapgs", options(nomem, preserves_flags)) };
    }
}
