use core::{fmt, iter, ops};

use crate::address::VirtAddr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(VirtAddr);
impl Page {
    pub const fn from_start(addr: VirtAddr) -> Option<Self> {
        if addr.as_u64() == Self::containing_addr(addr).0.as_u64() {
            Some(Page(addr))
        } else {
            None
        }
    }
    pub const fn containing_addr(addr: VirtAddr) -> Self {
        let addr = addr.as_u64() & !0xfff;
        Page(VirtAddr::new(addr))
    }

    pub const fn start(&self) -> VirtAddr {
        self.0
    }
    pub fn end(&self) -> VirtAddr {
        self.0 + 0xfff
    }

    pub fn p4_index(&self) -> usize {
        ((self.0.as_u64() >> 39) & 0x1ff) as usize
    }
    pub fn p3_index(&self) -> usize {
        ((self.0.as_u64() >> 30) & 0x1ff) as usize
    }
    pub fn p2_index(&self) -> usize {
        ((self.0.as_u64() >> 21) & 0x1ff) as usize
    }
    pub fn p1_index(&self) -> usize {
        ((self.0.as_u64() >> 12) & 0x1ff) as usize
    }
}
impl TryFrom<VirtAddr> for Page {
    type Error = &'static str;
    fn try_from(addr: VirtAddr) -> Result<Self, Self::Error> {
        Self::from_start(addr).ok_or("Invalid frame start address")
    }
}
impl fmt::Debug for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Page({:?})", self.0)
    }
}
impl ops::Add<u64> for Page {
    type Output = Self;
    fn add(self, rhs: u64) -> Self {
        Page::containing_addr(self.0 + rhs * 4096)
    }
}
impl ops::AddAssign<u64> for Page {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}
impl ops::Sub<u64> for Page {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self {
        Page::containing_addr(self.0 - rhs * 4096)
    }
}
impl ops::SubAssign<u64> for Page {
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

pub struct PageRange {
    pub start: Page,
    pub end: Page,
}
impl PageRange {
    pub const fn new(start: Page, end: Page) -> Self {
        PageRange { start, end }
    }
}
impl iter::Iterator for PageRange {
    type Item = Page;
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
