use core::fmt;

use x86_64::instructions::interrupts::without_interrupts;

use crate::logger::{DEBUG_CONSOLE, VIDEO_CONSOLE};

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    without_interrupts(|| {
        let mut debug_console = DEBUG_CONSOLE.lock();
        let mut video_console = VIDEO_CONSOLE.lock();
        let _ = debug_console.write_fmt(args);
        let _ = video_console.write_fmt(args);
    });
}

pub unsafe fn force_print(args: fmt::Arguments) {
    use fmt::Write;
    loop {
        if let Some(mut debug_console) = DEBUG_CONSOLE.try_lock() {
            let _ = debug_console.write_fmt(args);
            break;
        }
        // unsafe { DEBUG_CONSOLE.force_unlock() }
    }
    loop {
        if let Some(mut video_console) = VIDEO_CONSOLE.try_lock() {
            let _ = video_console.write_fmt(args);
            break;
        }
        // unsafe { VIDEO_CONSOLE.force_unlock() }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print(format_args!($($arg)*)));
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
            ref tmp => {
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
