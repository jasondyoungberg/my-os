use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{registers::segmentation::GS, structures::idt::InterruptStackFrame};

pub const PICS_OFFSET: u8 = 32;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new_contiguous(PICS_OFFSET) });

pub fn init() {
    let mut pics = PICS.lock();
    unsafe {
        pics.initialize();
        pics.disable();
    }
}

fn notify_end_of_interrupt(irq: u8) {
    let mut pics = PICS.try_lock().expect("Failed to get PICS lock");
    unsafe {
        pics.notify_end_of_interrupt(PICS_OFFSET + irq);
    }
}

pub fn pics_handler(_stack_frame: InterruptStackFrame, index: u8, _error_code: Option<u64>) {
    unsafe { GS::swap() };
    log::warn!("PIC interrupt: {}", index - PICS_OFFSET);
    let irq = index - PICS_OFFSET;
    crate::hardware::pics::notify_end_of_interrupt(irq);
    unsafe { GS::swap() };
}
