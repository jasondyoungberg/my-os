use log::{Level, Metadata, Record};

use crate::gsdata::CpuId;

static LOGGER: Logger = Logger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color_code = match record.level() {
                Level::Error => 91,
                Level::Warn => 93,
                Level::Info => 96,
                Level::Debug => 92,
                Level::Trace => 90,
            };

            if let Some(cpuid) = CpuId::find() {
                crate::kprintln!("\x1b[{}m[{}] {}\x1b[0m", color_code, cpuid, record.args());
            } else {
                crate::kprintln!("\x1b[{}m[CPU?] {}\x1b[0m", color_code, record.args());
            }
        }
    }

    fn flush(&self) {}
}
