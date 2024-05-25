use x86_64::structures::idt::InterruptStackFrame;

use crate::threading::state::ProcessState;

#[naked]
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // code based on https://github.com/rust-osdev/x86_64/issues/392#issuecomment-1257883895
    unsafe {
        core::arch::asm! {"
                push r15
                push r14
                push r13
                push r12
                push r11
                push r10
                push r9
                push r8
                push 0 // skip rsp
                push rbp
                push rdi
                push rsi
                push rdx
                push rcx
                push rbx
                push rax

                // 1st arg (rdi): stack_frame
                // 2nd arg (rsi): registers
                lea rdi, [rsp + 8*16]
                mov rdi, rsp
                call {}

                pop rax
                pop rbx
                pop rcx
                pop rdx
                pop rsi
                pop rdi
                pop rbp
                add rsp, 8 // skip rsp
                pop r8
                pop r9
                pop r10
                pop r11
                pop r12
                pop r13
                pop r14
                pop r15

                iretq
            ",
            sym breakpoint_handler_inner,
            options(noreturn)
        };
    }
}

extern "C" fn breakpoint_handler_inner(state: &mut ProcessState) {
    log::info!("Breakpoint");

    let mut manager = crate::threading::manager::MANAGER.lock();

    manager.next(state);
}
