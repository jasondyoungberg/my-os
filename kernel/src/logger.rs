use core::fmt::Write;

use log::{Level, Log};
use spin::{Lazy, Mutex};

use crate::{
    drivers::{
        debug_console::DebugConsole,
        display::{Color, Display},
        video_console::{self, VideoConsole},
    },
    FRAMEBUFFER_RESPONSE,
};

static LOGGER: Logger = Logger;
pub static DEBUG_CONSOLE: Mutex<DebugConsole> = Mutex::new(DebugConsole);
pub static VIDEO_CONSOLE: Lazy<Mutex<VideoConsole>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_RESPONSE.framebuffers().next().unwrap();
    let console = VideoConsole::new(Display::new(&framebuffer));
    Mutex::new(console)
});

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &log::Record) {
        let color_code = match record.level() {
            Level::Error => 91,
            Level::Warn => 93,
            Level::Info => 96,
            Level::Debug => 92,
            Level::Trace => 90,
        };

        let mut debug_console = DEBUG_CONSOLE.lock();
        let _ =
            debug_console.write_fmt(format_args!("\x1b[{color_code}m{}\x1b[0m\n", record.args()));

        if record.level() <= Level::Debug {
            let mut video_console = VIDEO_CONSOLE.lock();

            video_console.color_fg = match record.level() {
                Level::Error => Color::new(255, 0, 0),
                Level::Warn => Color::new(255, 255, 0),
                Level::Info => Color::new(0, 255, 255),
                Level::Debug => Color::new(0, 255, 0),
                Level::Trace => Color::new(128, 128, 128),
            };

            let _ = video_console.write_fmt(format_args!("{}\n", record.args()));

            video_console.color_fg = Color::new(255, 255, 255);
        }
    }

    fn flush(&self) {}
}
