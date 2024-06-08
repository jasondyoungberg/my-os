use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

use crate::gsdata::GsData;

pub struct Ps2;

impl Ps2 {}

impl Ps2 {}

pub extern "x86-interrupt" fn keyboard_interrupt(_stack_frame: InterruptStackFrame) {
    let scancode = unsafe { Port::<u8>::new(0x60).read() };
    log::info!("scancode {:#x}", scancode);
    GsData::load()
        .expect("Unable to load gsdata")
        .lapic
        .lock()
        .signal_eoi();
}
