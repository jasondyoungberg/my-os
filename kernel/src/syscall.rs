use x86_64::{
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask, Star},
        rflags::RFlags,
    },
    VirtAddr,
};

use crate::{gdt::GDT, process::Registers, wrap};

pub fn init() {
    LStar::write(VirtAddr::new(handle_syscall as usize as u64));
    Star::write(
        GDT.user_code,
        GDT.user_data,
        GDT.kernel_code,
        GDT.kernel_data,
    )
    .unwrap();
    SFMask::write(RFlags::INTERRUPT_FLAG);

    let efer = Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS;

    unsafe { Efer::write(efer) };
}

wrap!(syscall, handle_syscall_inner => handle_syscall);

extern "C" fn handle_syscall_inner(_registers: &mut Registers) {
    todo!();
}
