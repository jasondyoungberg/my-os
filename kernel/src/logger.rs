use log::{Level, Metadata, Record};

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
