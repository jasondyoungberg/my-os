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

    pub const fn align_up(&self, align: u64) -> Self {
        VirtAddr::new_truncate(align_up(self.0, align))
    }

    pub const fn align_down(&self, align: u64) -> Self {
        VirtAddr::new_truncate(align_down(self.0, align))
    }
}

const fn align_up(addr: u64, align: u64) -> u64 {
    (addr + align - 1) & !(align - 1)
}

const fn align_down(addr: u64, align: u64) -> u64 {
    addr & !(align - 1)
}
