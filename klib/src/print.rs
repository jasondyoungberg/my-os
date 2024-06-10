use core::arch::asm;

struct DebugConsole;

impl core::fmt::Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            // Safety: This is safe because we are writing to qemu's debug port.
            unsafe { asm!("out 0xe9, al", in("al") byte) };
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _kprint(args: core::fmt::Arguments) {
    use core::fmt::Write;
    DebugConsole
        .write_fmt(args)
        .expect("Console should never return an error");
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        $crate::print::_kprint(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::kprint!("\n")
    };
    ($($arg:tt)*) => {
        $crate::kprint!("{}\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::kprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::kprintln!("[{}:{}:{}] {} = {:#?}",
                    file!(), line!(), column!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
