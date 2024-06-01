use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") value) };
}

pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe { asm!("in al, dx", out("al") value, in("dx") port) };
    value
}

pub unsafe fn outw(port: u16, value: u16) {
    unsafe { asm!("out dx, ax", in("dx") port, in("ax") value) };
}

pub unsafe fn inw(port: u16) -> u16 {
    let value: u16;
    unsafe { asm!("in ax, dx", out("ax") value, in("dx") port) };
    value
}

pub unsafe fn outl(port: u16, value: u32) {
    unsafe { asm!("out dx, eax", in("dx") port, in("eax") value) };
}

pub unsafe fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe { asm!("in eax, dx", out("eax") value, in("dx") port) };
    value
}
