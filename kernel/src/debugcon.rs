use core::fmt;

use x86_64::instructions::port::PortWriteOnly;

struct Writer();

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

#[doc(hidden)]
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
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::println!("[{}:{}:{}] {} = {:#?}",
                    file!(), line!(), column!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
