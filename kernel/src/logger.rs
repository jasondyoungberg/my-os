use log::{Level, Metadata, Record};
use x86_64::registers::model_specific::GsBase;

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

            let core_data_addr = GsBase::read();

            if core_data_addr >= x86_64::VirtAddr::new(0xFFFF_FFFF_8000_0000) {
                let core_data_ptr = core_data_addr.as_u64() as *const crate::core::CoreData;
                let core_data = unsafe { &*core_data_ptr };

                crate::kprintln!(
                    "\x1b[{}m[CPU{}] {}\x1b[0m",
                    color_code,
                    core_data.id,
                    record.args()
                );
            } else {
                crate::kprintln!("\x1b[{}m[CPU?] {}\x1b[0m", color_code, record.args());
            }
        }
    }

    fn flush(&self) {}
}
