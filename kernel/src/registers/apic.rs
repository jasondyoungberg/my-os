use crate::{address::PhysAddr, println, structures::paging::PhysFrame};

use super::ModelSpecificRegister;

pub struct ApicBase;
impl ApicBase {
    const MSR_ADDR: u32 = 0x1B;

    pub fn is_enabled() -> bool {
        Self::read() & (1 << 11) != 0
    }

    pub fn is_bsp() -> bool {
        Self::read() & (1 << 8) != 0
    }

    pub fn enable() {
        let mut value = Self::read();
        value |= 1 << 11;
        Self::write(value);
    }

    pub fn disable() {
        let mut value = Self::read();
        value &= !(1 << 11);
        Self::write(value);
    }

    pub fn get_base() -> PhysFrame {
        PhysFrame::containing_addr(PhysAddr::new_truncate(Self::read()))
    }

    pub fn set_base(base: PhysFrame) {
        let mut value = Self::read();
        value &= 0x0FFF;
        value |= base.start().as_u64();
        Self::write(value);
    }

    pub fn read() -> u64 {
        ModelSpecificRegister::<{ Self::MSR_ADDR }>::read()
    }

    pub fn write(value: u64) {
        ModelSpecificRegister::<{ Self::MSR_ADDR }>::write(value)
    }
}