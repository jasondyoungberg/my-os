use crate::address::VirtAddr;

pub struct Mmio {
    base: VirtAddr,
    size: usize,
}

impl Mmio {
    pub unsafe fn new(base: VirtAddr, size: usize) -> Self {
        Mmio { base, size }
    }

    pub fn read<T>(&self, offset: usize) -> T {
        assert!(offset + core::mem::size_of::<T>() <= self.size);
        let ptr = (self.base.as_u64() + offset as u64) as *const T;
        unsafe { ptr.read_volatile() }
    }

    pub fn write<T>(&self, offset: usize, value: T) {
        assert!(offset + core::mem::size_of::<T>() <= self.size);
        let ptr = (self.base.as_u64() + offset as u64) as *mut T;
        unsafe { ptr.write_volatile(value) }
    }
}
