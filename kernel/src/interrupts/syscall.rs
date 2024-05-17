use x86_64::structures::idt::InterruptStackFrame;

use crate::{info, trace};
use stdlib::syscall::{Error, Request, Response};

#[naked]
pub extern "x86-interrupt" fn syscall_handler(stack_frame: InterruptStackFrame) {
    // code based on https://github.com/rust-osdev/x86_64/issues/392#issuecomment-1257883895
    unsafe {
        core::arch::asm! {"
                // Save the registers
                push rax
                push rcx
                push rdx
                push r8
                push r9
                push r10
                push r11

                mov rdi, rdx    // 1st arg: syscall output
                mov rsi, rsi    // 2nd arg: syscall input
                mov rdx, rsp    // 3rd arg: interrupt frame
                call {}         // Call the actual handler

                // Restore the registers
                pop r11
                pop r10
                pop r9
                pop r8
                pop rdx
                pop rcx
                pop rax

                iretq // return
            ",
            sym syscall_handler_inner,
            options(noreturn)
        };
    }
}

extern "C" fn syscall_handler_inner(
    response: &mut Result<Response, Error>,
    request: &Request,
    _stack_frame: &InterruptStackFrame,
) {
    *response = match request {
        Request::Print(x) => print(x).map(|()| Response::Print),
    };

    trace!("syscall {request:?} => {response:?}");
}

/// Handle a print syscall
#[allow(clippy::unnecessary_wraps)]
fn print(msg: &str) -> Result<(), Error> {
    info!("{}", msg);
    Ok(())
}
