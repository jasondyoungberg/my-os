use core::arch::asm;

use x86_64::{
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask, Star},
        rflags::RFlags,
    },
    VirtAddr,
};

use crate::{gdt::GDT, println, process::Registers};

pub fn init() {
    LStar::write(VirtAddr::new(handle_syscall as usize as u64));
    Star::write(
        GDT.user_code,
        GDT.user_data,
        GDT.kernel_code,
        GDT.kernel_data,
    )
    .unwrap();
    SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG);

    unsafe { Efer::write(Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS) };
}

#[naked]
extern "C" fn handle_syscall(registers: &mut Registers) {
    unsafe {
        asm!(
            "
            swapgs
            mov gs:24, rsp
            mov rsp, gs:16

            push r15
            push r14
            push r13
            push r12
            push r11
            push r10
            push r9
            push r8
            push rbp
            push rdi
            push rsi
            push rdx
            push rcx
            push rbx
            push rax

            mov rdi, rsp
            call {inner}

            pop rax
            pop rbx
            pop rcx
            pop rdx
            pop rsi
            pop rdi
            pop rbp
            pop r8
            pop r9
            pop r10
            pop r11
            pop r12
            pop r13
            pop r14
            pop r15

            mov rsp, gs:24
            swapgs
            sysretq
            ",
            inner = sym handle_syscall_inner,
            options(noreturn)
        );
    }
}

extern "C" fn handle_syscall_inner(registers: &mut Registers) {
    let num = registers.rax;
    let arg1 = registers.rdi;
    let arg2 = registers.rsi;
    let arg3 = registers.rdx;
    let arg4 = registers.r10;
    let arg5 = registers.r8;
    let arg6 = registers.r9;

    println!("syscall {num} ({arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6})");
}
