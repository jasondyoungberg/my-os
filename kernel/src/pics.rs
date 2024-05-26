use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

pub const PICS_OFFSET: u8 = 32;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new_contiguous(PICS_OFFSET) });

pub fn init() {
    let mut pics = PICS.lock();
    unsafe {
        pics.initialize();
        pics.write_masks(0, 0);
    }
}

fn notify_end_of_interrupt(irq: u8) {
    let mut pics = PICS.try_lock().expect("Failed to get PICS lock");
    unsafe {
        pics.notify_end_of_interrupt(PICS_OFFSET + irq);
    }
}

pub extern "x86-interrupt" fn handle_irq0(_stack_frame: InterruptStackFrame) {
    log::trace!("timer interrupt");
    notify_end_of_interrupt(0);
}

pub extern "x86-interrupt" fn handle_irq1(_stack_frame: InterruptStackFrame) {
    log::trace!("keyboard interrupt");
    notify_end_of_interrupt(1);
}

pub extern "x86-interrupt" fn handle_irq2(_stack_frame: InterruptStackFrame) {
    log::trace!("cascade interrupt");
    notify_end_of_interrupt(2);
}

pub extern "x86-interrupt" fn handle_irq3(_stack_frame: InterruptStackFrame) {
    log::trace!("COM2 interrupt");
    notify_end_of_interrupt(3);
}

pub extern "x86-interrupt" fn handle_irq4(_stack_frame: InterruptStackFrame) {
    log::trace!("COM1 interrupt");
    notify_end_of_interrupt(4);
}

pub extern "x86-interrupt" fn handle_irq5(_stack_frame: InterruptStackFrame) {
    log::trace!("LPT2 interrupt");
    notify_end_of_interrupt(5);
}

pub extern "x86-interrupt" fn handle_irq6(_stack_frame: InterruptStackFrame) {
    log::trace!("floppy disk interrupt");
    notify_end_of_interrupt(6);
}

pub extern "x86-interrupt" fn handle_irq7(_stack_frame: InterruptStackFrame) {
    log::trace!("LPT1 interrupt");
    notify_end_of_interrupt(7);
}

pub extern "x86-interrupt" fn handle_irq8(_stack_frame: InterruptStackFrame) {
    log::trace!("CMOS interrupt");
    notify_end_of_interrupt(8);
}

pub extern "x86-interrupt" fn handle_irq9(_stack_frame: InterruptStackFrame) {
    log::trace!("acpi interrupt");
    notify_end_of_interrupt(9);
}

pub extern "x86-interrupt" fn handle_irq10(_stack_frame: InterruptStackFrame) {
    log::trace!("open interrupt");
    notify_end_of_interrupt(10);
}

pub extern "x86-interrupt" fn handle_irq11(_stack_frame: InterruptStackFrame) {
    log::trace!("open interrupt");
    notify_end_of_interrupt(11);
}

pub extern "x86-interrupt" fn handle_irq12(_stack_frame: InterruptStackFrame) {
    log::trace!("mouse interrupt");
    notify_end_of_interrupt(12);
}

pub extern "x86-interrupt" fn handle_irq13(_stack_frame: InterruptStackFrame) {
    log::trace!("FPU interrupt");
    notify_end_of_interrupt(13);
}

pub extern "x86-interrupt" fn handle_irq14(_stack_frame: InterruptStackFrame) {
    log::trace!("primary ATA interrupt");
    notify_end_of_interrupt(14);
}

pub extern "x86-interrupt" fn handle_irq15(_stack_frame: InterruptStackFrame) {
    log::trace!("secondary ATA interrupt");
    notify_end_of_interrupt(15);
}
