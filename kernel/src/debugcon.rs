use core::fmt;
use spin::Mutex;

use x86_64::instructions::{interrupts::without_interrupts, port::PortWriteOnly};

static WRITER: Mutex<Writer> = Mutex::new(Writer);

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    without_interrupts(|| WRITER.lock().write_fmt(args).unwrap());
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
