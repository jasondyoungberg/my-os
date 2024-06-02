use core::fmt;

use x86_64::instructions::port::PortWriteOnly;

pub struct DebugConsole;

impl fmt::Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut port = PortWriteOnly::new(0xe9);
        for b in s.bytes() {
            unsafe { port.write(b) };
        }
        Ok(())
    }
}
