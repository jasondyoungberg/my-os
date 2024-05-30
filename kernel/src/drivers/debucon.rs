pub fn print(msg: &str) {
    for b in msg.bytes() {
        unsafe { crate::instructions::outb(0xe9, b as u8) };
    }
}
