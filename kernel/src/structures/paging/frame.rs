use crate::address::PhysAddr;

pub struct PhysFrame(PhysAddr);
impl PhysFrame {
    pub fn new(addr: PhysAddr) -> Self {
        PhysFrame(addr)
    }
    pub fn try_new(addr: PhysAddr) -> Option<Self> {
        if addr == Self::new_truncate(addr).0 {
            Some(PhysFrame(addr))
        } else {
            None
        }
    }
    pub fn new_truncate(addr: PhysAddr) -> Self {
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
