use crate::{address::PhysAddr, structures::paging::PhysFrame};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default)]
    #[repr(transparent)]
    pub struct Cr3Flags: u64 {
        const WRITE_THROUGH = 1 << 3;
        const CACHE_DISABLE = 1 << 4;
    }
}

pub struct Cr3;
impl Cr3 {
    pub fn read() -> (PhysFrame, Cr3Flags) {
        let value: u64;
        unsafe { core::arch::asm!("mov {}, cr3", out(reg) value, options(nomem, preserves_flags)) };
        let addr = PhysAddr::new(value & 0x_ffff_ffff_ffff_f000);
        let frame = PhysFrame::from_start(addr).unwrap();
        let flags = Cr3Flags::from_bits_truncate(value);
        (frame, flags)
    }

    pub unsafe fn write(frame: PhysFrame, flags: Cr3Flags) {
        let value = frame.start().as_u64() | flags.bits();
        unsafe { core::arch::asm!("mov cr3, {}", in(reg) value, options(nomem)) };
    }
}
