use core::arch::asm;

/// Writes a byte to an I/O port
///
/// # Safety
/// Modifying I/O ports can have unintended side effects, and is inherently unsafe.
pub unsafe fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val);
    }
}

/// Writes 2 bytes to an I/O port
///
/// # Safety
/// Modifying I/O ports can have unintended side effects, and is inherently unsafe.
pub unsafe fn outw(port: u16, val: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") val);
    }
}

/// Writes 4 bytes to an I/O port
///
/// # Safety
/// Modifying I/O ports can have unintended side effects, and is inherently unsafe.
pub unsafe fn outl(port: u16, val: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") val);
    }
}

/// Reads a byte from an I/O port
///
/// # Safety
/// Reading from I/O ports can have unintended side effects, and is inherently unsafe.
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        asm!("in al, dx", out("al") ret, in("dx") port);
    }
    ret
}

/// Reads 2 bytes from an I/O port
///
/// # Safety
/// Reading from I/O ports can have unintended side effects, and is inherently unsafe.
pub unsafe fn inw(port: u16) -> u16 {
    let ret: u16;
    unsafe {
        asm!("in ax, dx", out("ax") ret, in("dx") port);
    }
    ret
}

/// Reads 4 bytes from an I/O port
///
/// # Safety
/// Reading from I/O ports can have unintended side effects, and is inherently unsafe.
pub unsafe fn inl(port: u16) -> u32 {
    let ret: u32;
    unsafe {
        asm!("in eax, dx", out("eax") ret, in("dx") port);
    }
    ret
}
