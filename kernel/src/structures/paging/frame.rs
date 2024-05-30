use core::{fmt, iter, ops};

use crate::address::PhysAddr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysFrame(PhysAddr);
impl PhysFrame {
    pub fn from_start(addr: PhysAddr) -> Option<Self> {
        if addr == Self::containing_addr(addr).0 {
            Some(PhysFrame(addr))
        } else {
            None
        }
    }
    pub fn containing_addr(addr: PhysAddr) -> Self {
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
impl ops::Add<u64> for PhysFrame {
    type Output = Self;
    fn add(self, rhs: u64) -> Self {
        PhysFrame::containing_addr(self.0 + rhs * 4096)
    }
}
impl ops::AddAssign<u64> for PhysFrame {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}
impl ops::Sub<u64> for PhysFrame {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self {
        PhysFrame::containing_addr(self.0 - rhs * 4096)
    }
}
impl ops::SubAssign<u64> for PhysFrame {
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

pub struct PhysFrameRange {
    start: PhysFrame,
    end: PhysFrame,
}
impl PhysFrameRange {
    pub fn new(start: PhysFrame, end: PhysFrame) -> Self {
        PhysFrameRange { start, end }
    }
}
impl iter::Iterator for PhysFrameRange {
    type Item = PhysFrame;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let frame = self.start;
            self.start += 1;
            Some(frame)
        } else {
            None
        }
    }
}
