use core::fmt;

pub struct DebugConsole;

impl fmt::Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            unsafe { crate::instructions::outb(0xe9, b) };
        }
        Ok(())
    }
}
