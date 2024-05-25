use core::fmt;

use x86_64::instructions::port::PortWriteOnly;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use fmt::Write;
    Writer.write_fmt(args).unwrap();
}

struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut port = PortWriteOnly::new(0xE9);
        for byte in s.bytes() {
            unsafe {
                port.write(byte);
            }
        }
        Ok(())
    }
}