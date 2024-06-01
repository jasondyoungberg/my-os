use crate::{address::PhysAddr, structures::paging::PhysFrame};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default)]
    #[repr(transparent)]
    pub struct Cr0Flags: u64 {
        const PROTECTION_ENABLE = 1 << 0;
        const MONITOR_COPROCESSOR = 1 << 1;
        const EMULATION = 1 << 2;
        const TASK_SWITCHED = 1 << 3;
        const EXTENSION_TYPE = 1 << 4;
        const NUMERIC_ERROR = 1 << 5;
        const WRITE_PROTECT = 1 << 16;
        const ALIGNMENT_MASK = 1 << 18;
        const NOT_WRITE_THROUGH = 1 << 29;
        const CACHE_DISABLE = 1 << 30;
        const PAGING = 1 << 31;
    }
}

pub struct Cr0;
impl Cr0 {
    pub fn read() -> Cr0Flags {
        let flags: u64;
        unsafe { core::arch::asm!("mov {}, cr0", out(reg) flags, options(nomem, preserves_flags)) };
        Cr0Flags::from_bits_retain(flags)
    }

    pub unsafe fn write(flags: Cr0Flags) {
        unsafe { core::arch::asm!("mov cr0, {}", in(reg) flags.bits(), options(nomem)) };
    }
}
