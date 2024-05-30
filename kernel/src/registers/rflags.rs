bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default)]
    #[repr(transparent)]
    pub struct RFlags: u64 {
        const CARRY = 1 << 0;
        const _ = 1 << 1;
        const PARITY = 1 << 2;
        const ADJUST = 1 << 4;
        const ZERO = 1 << 6;
        const SIGN = 1 << 7;
        const TRAP = 1 << 8;
        const INTERRUPT = 1 << 9;
        const DIRECTION = 1 << 10;
        const OVERFLOW = 1 << 11;
        const IOPL_LOW = 1 << 12;
        const IOPL_HIGH = 1 << 13;
        const NESTED_TASK = 1 << 14;
        const RESUME = 1 << 16;
        const VIRTUAL_8086 = 1 << 17;
        const ALIGNMENT_CHECK = 1 << 18;
        const VIRTUAL_INTERRUPT = 1 << 19;
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        const ID = 1 << 21;
    }
}
impl RFlags {
    pub fn read() -> Self {
        let rflags: u64;
        unsafe {
            core::arch::asm!("pushfq; pop {}", out(reg) rflags, options(nomem, preserves_flags))
        };
        Self::from_bits_retain(rflags)
    }

    pub unsafe fn write(self) {
        let rflags = self.bits();
        unsafe { core::arch::asm!("push {}; popfq", in(reg) rflags, options(nomem)) };
    }
}
