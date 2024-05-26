use alloc::boxed::Box;
use x2apic::lapic::LocalApic;
use x86_64::registers::model_specific::GsBase;

#[derive(Debug)]
pub struct CoreData {
    pub id: u32,
    pub lapic: Box<LocalApic>,
}

pub fn get_core_data() -> Option<&'static mut CoreData> {
    let core_data_addr = GsBase::read();
    if core_data_addr >= x86_64::VirtAddr::new(0xFFFF_FFFF_8000_0000) {
        let core_data_ptr = core_data_addr.as_u64() as *mut CoreData;
        Some(unsafe { &mut *core_data_ptr })
    } else {
        None
    }
}
