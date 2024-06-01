use crate::{
    address::{PhysAddr, VirtAddr},
    structures::paging::PhysFrame,
};

pub struct Cr2;
impl Cr2 {
    pub fn read() -> VirtAddr {
        let value: u64;
        unsafe { core::arch::asm!("mov {}, cr2", out(reg) value, options(nomem, preserves_flags)) };
        VirtAddr::new(value)
    }
}
