use x2apic::lapic::{LocalApic, LocalApicBuilder, TimerDivide};
use x86_64::registers::segmentation::GS;

use crate::core::get_core_data;

pub const TIMER_VECTOR: u8 = 0x40;
pub const ERROR_VECTOR: u8 = 0x41;
pub const SPURIOUS_VECTOR: u8 = 0xFF;

pub fn init() -> LocalApic {
    let mut builder = LocalApicBuilder::new();

    builder.timer_vector(TIMER_VECTOR as usize);
    builder.error_vector(ERROR_VECTOR as usize);
    builder.spurious_vector(SPURIOUS_VECTOR as usize);

    builder.timer_initial(1_000_000_000);
    builder.timer_divide(TimerDivide::Div256);

    let mut lapic = builder.build().unwrap();
    unsafe { lapic.enable() };
    lapic
}

pub extern "x86-interrupt" fn handle_timer(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    unsafe { GS::swap() };
    log::trace!("timer interrupt");
    let cpu_data = get_core_data().unwrap();
    unsafe { cpu_data.lapic.end_of_interrupt() };
    unsafe { GS::swap() };
}
