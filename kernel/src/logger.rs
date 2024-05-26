use log::{Level, Metadata, Record};

use crate::gsdata::get_kernel_gs_data;

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

            if let Some(core_data) = get_kernel_gs_data() {
                crate::kprintln!(
                    "\x1b[{}m[CPU{}] {}\x1b[0m",
                    color_code,
                    core_data.cpuid,
                    record.args()
                );
            } else {
                crate::kprintln!("\x1b[{}m[CPU?] {}\x1b[0m", color_code, record.args());
            }
        }
    }

    fn flush(&self) {}
}
