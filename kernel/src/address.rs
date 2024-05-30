use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(u64);
impl VirtAddr {
    pub const fn new(addr: u64) -> Self {
        match Self::try_new(addr) {
            Some(addr) => addr,
            None => panic!("Invalid virtual address"),
        }
    }
    pub const fn try_new(addr: u64) -> Option<Self> {
        if addr == Self::new_truncate(addr).0 {
            Some(VirtAddr(addr))
        } else {
            None
        }
    }
    pub const fn new_truncate(addr: u64) -> Self {
        VirtAddr(((addr << 16) as i64 >> 16) as u64)
    }
    pub const fn null() -> Self {
        Self::new(0)
    }
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
    pub const fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }
    pub const fn as_mut_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }
    pub fn from_ptr<T: ?Sized>(ptr: *const T) -> Self {
        Self::new(ptr as *const () as u64)
    }
    pub fn from_mut_ptr<T: ?Sized>(ptr: *mut T) -> Self {
        Self::new(ptr as *const () as u64)
    }
    pub fn align_up(&self, align: u64) -> Self {
        VirtAddr::new_truncate(align_up(self.0, align))
    }
    pub fn align_down(&self, align: u64) -> Self {
        VirtAddr::new_truncate(align_down(self.0, align))
    }
}
impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VirtAddr({:#x})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(u64);
impl PhysAddr {
    pub const fn new(addr: u64) -> Self {
        match Self::try_new(addr) {
            Some(addr) => addr,
            None => panic!("Invalid physical address"),
        }
    }
    pub const fn try_new(addr: u64) -> Option<Self> {
        if addr == Self::new_truncate(addr).0 {
            Some(PhysAddr(addr))
        } else {
            None
        }
    }
    pub const fn new_truncate(addr: u64) -> Self {
        PhysAddr(addr % (1 << 52))
    }
    pub const fn null() -> Self {
        Self::new(0)
    }
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
    pub fn align_up(&self, align: u64) -> Self {
        PhysAddr::new(align_up(self.0, align))
    }
    pub fn align_down(&self, align: u64) -> Self {
        PhysAddr::new(align_down(self.0, align))
    }
}
impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysAddr({:#x})", self.0)
    }
}

fn align_up(addr: u64, align: u64) -> u64 {
    assert_eq!(align.count_ones(), 1, "Alignment must be a power of 2");
    (addr + align - 1) & !(align - 1)
}
fn align_down(addr: u64, align: u64) -> u64 {
    assert_eq!(align.count_ones(), 1, "Alignment must be a power of 2");
    addr & !(align - 1)
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn virtaddr_new_lower(addr in 0..0x7fff_ffff_ffffu64) {
            let virt = crate::address::VirtAddr::new(addr);
            assert_eq!(virt.as_u64(), addr);
        }
        #[test]
        fn virtaddr_new_upper(addr in 0xffff_8000_0000_0000..u64::MAX) {
            let virt = crate::address::VirtAddr::new(addr);
            assert_eq!(virt.as_u64(), addr);
        }
        #[test]
        fn physaddr_new(addr in 0..0x000f_ffff_ffff_ffffu64) {
            let phys = crate::address::PhysAddr::new(addr);
            assert_eq!(phys.as_u64(), addr);
        }

        #[test]
        fn align_up(
            addr in 0..=u64::MAX,
            align_pow in 0..52u64,
        ) {
            let align = 1 << align_pow;
            let aligned = crate::address::align_up(addr, align);
            assert_eq!(aligned % align, 0);
            assert!(aligned >= addr);
        }
        #[test]
        fn align_down(
            addr in 0..=u64::MAX,
            align_pow in 0..52u64,
        ) {
            let align = 1 << align_pow;
            let aligned = crate::address::align_down(addr, align);
            assert_eq!(aligned % align, 0);
            assert!(aligned <= addr);
        }
    }
}
