mod io;

pub use io::*;

use core::arch::asm;

use crate::{
    address::VirtAddr,
    registers::{Cr3, RFlags},
};

pub fn hlt() {
    unsafe { asm!("hlt") };
}

pub fn breakpoint() {
    unsafe { asm!("int3") };
}

pub fn enable_interrupts() {
    unsafe { asm!("sti") };
}

pub fn disable_interrupts() {
    unsafe { asm!("cli") };
}

pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    if RFlags::read().contains(RFlags::INTERRUPT) {
        disable_interrupts();
        let res = f();
        enable_interrupts();
        res
    } else {
        f()
    }
}

pub fn flush_tlb(addr: VirtAddr) {
    unsafe { asm!("invlpg [{}]", in(reg) addr.as_u64()) };
}

pub fn flush_tlb_all() {
    let (frame, flags) = Cr3::read();
    unsafe {
        Cr3::write(frame, flags);
    }
}
