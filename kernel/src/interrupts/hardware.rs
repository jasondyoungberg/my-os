use crate::keyboard;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    PrimaryAta = PIC_2_OFFSET + 6,
}

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn load() {
    unsafe {
        PICS.lock().initialize();
    }
}

fn end_interrupt(index: InterruptIndex) {
    unsafe {
        PICS.try_lock()
            .expect("Failed to get PICS lock")
            .notify_end_of_interrupt(index as u8);
    }
}

pub extern "x86-interrupt" fn timer_interrupt(_stack_frame: InterruptStackFrame) {
    use core::time::Duration;

    const FREQUENCY: f64 = 1_193_181.666_666;
    const DIVIDER: f64 = 65_536.0;
    const NS_PER_S: f64 = 1_000_000_000.0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    const NS_PER_TICK: u64 = (NS_PER_S / (FREQUENCY / DIVIDER)) as u64;

    crate::task::delay::add_time(Duration::from_nanos(NS_PER_TICK));

    end_interrupt(InterruptIndex::Timer);
}

pub extern "x86-interrupt" fn keyboard_interrupt(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let scancode: u8 = unsafe { Port::new(0x60).read() };

    keyboard::add_scancode(scancode);

    end_interrupt(InterruptIndex::Keyboard);
}

pub extern "x86-interrupt" fn primary_ata_interrupt(_stack_frame: InterruptStackFrame) {
    crate::disk::ata::wake();

    end_interrupt(InterruptIndex::PrimaryAta);
}
