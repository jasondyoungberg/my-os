use core::fmt;

use log::{Level, Metadata, Record};
use x86_64::instructions::port::PortWriteOnly;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use fmt::Write;
    Writer.write_fmt(args).unwrap();
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

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let color_code = match level {
                Level::Error => 91,
                Level::Warn => 93,
                Level::Info => 96,
                Level::Debug => 92,
                Level::Trace => 90,
            };

            crate::kprintln!("\x1b[{color_code}m{level}: {}\x1b[0m", record.args());
        }
    }

    fn flush(&self) {}
}
