use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder, TimerDivide};

use crate::{
    gsdata::KernelData,
    mapper::map_mmio,
    process::{Context, MANAGER},
    wrap,
};

pub const TIMER_VECTOR: u8 = 0x40;
pub const ERROR_VECTOR: u8 = 0x41;
pub const SPURIOUS_VECTOR: u8 = 0xFF;

pub fn init() -> LocalApic {
    let mut builder = LocalApicBuilder::new();

    let apic_phys_addr = unsafe { xapic_base() };
    let apic_virt_addr = map_mmio(apic_phys_addr, 4096);

    builder.set_xapic_base(apic_virt_addr.as_u64());

    builder.timer_vector(TIMER_VECTOR as usize);
    builder.error_vector(ERROR_VECTOR as usize);
    builder.spurious_vector(SPURIOUS_VECTOR as usize);

    builder.timer_initial(1_000_000_000);
    builder.timer_divide(TimerDivide::Div256);

    let mut lapic = builder.build().unwrap();
    unsafe { lapic.enable() };
    lapic
}

wrap!(irq, handle_timer_inner => handle_timer);

extern "C" fn handle_timer_inner(context: &mut Context) {
    log::trace!("timer interrupt");
    let cpu_data = KernelData::load_gsbase().unwrap();

    MANAGER.get().unwrap().lock().swap_task(cpu_data, context);

    unsafe { cpu_data.lapic.end_of_interrupt() };
}
