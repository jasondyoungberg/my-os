use core::fmt;
use spin::Mutex;

use x86_64::instructions::{interrupts::without_interrupts, port::PortWriteOnly};

pub static WRITER: Mutex<DebugWriter> = Mutex::new(DebugWriter);

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    without_interrupts(|| WRITER.lock().write_fmt(args).unwrap());
    // Writer.write_fmt(args).unwrap();
}

pub struct DebugWriter;

impl fmt::Write for DebugWriter {
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
