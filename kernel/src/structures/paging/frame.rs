use core::fmt;

use crate::address::PhysAddr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PhysFrame(PhysAddr);
impl PhysFrame {
    pub fn from_start(addr: PhysAddr) -> Option<Self> {
        if addr == Self::containing(addr).0 {
            Some(PhysFrame(addr))
        } else {
            None
        }
    }
    pub fn containing(addr: PhysAddr) -> Self {
        let addr = addr.as_u64() & !0xfff;
        PhysFrame(PhysAddr::new(addr))
    }

    pub fn start(&self) -> PhysAddr {
        self.0
    }
    pub fn end(&self) -> PhysAddr {
        self.0 + 0xfff
    }
}
impl TryFrom<PhysAddr> for PhysFrame {
    type Error = &'static str;
    fn try_from(addr: PhysAddr) -> Result<Self, Self::Error> {
        Self::from_start(addr).ok_or("Invalid frame start address")
    }
}
impl fmt::Debug for PhysFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysFrame({:?})", self.0)
    }
}
