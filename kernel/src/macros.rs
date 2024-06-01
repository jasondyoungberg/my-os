use core::fmt;

use spin::{Lazy, Mutex};

use crate::{
    drivers::{debug_console::DebugConsole, display::Display, video_console::VideoConsole},
    instructions::without_interrupts,
    FRAMEBUFFER_REQUEST,
};

static DEBUG_CONSOLE: Mutex<DebugConsole> = Mutex::new(DebugConsole);
static VIDEO_CONSOLE: Lazy<Mutex<VideoConsole>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_REQUEST
        .response
        .get()
        .unwrap()
        .framebuffers()
        .next()
        .unwrap();
    let console = VideoConsole::new(Display::new(framebuffer));
    Mutex::new(console)
});

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
    let _ = DebugConsole.write_fmt(args);
    loop {
        if let Some(mut video_console) = VIDEO_CONSOLE.try_lock() {
            let _ = video_console.write_fmt(args);
            return;
        }
        unsafe { VIDEO_CONSOLE.force_unlock() }
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
