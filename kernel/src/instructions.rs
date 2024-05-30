use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value);
}

pub fn hlt() {
    unsafe { asm!("hlt") };
}
