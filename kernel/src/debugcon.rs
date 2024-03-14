use core::fmt;

struct Writer();

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                cpu::io::outb(0xE9, byte);
            }
        }
        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
    use fmt::Write;
    Writer().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::debugcon::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
