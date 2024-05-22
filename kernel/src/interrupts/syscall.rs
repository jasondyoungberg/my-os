use x86_64::structures::idt::InterruptStackFrame;

use stdlib::syscall::{Error, Request, Response};

#[naked]
pub extern "x86-interrupt" fn syscall_handler(stack_frame: InterruptStackFrame) {
    // code based on https://github.com/rust-osdev/x86_64/issues/392#issuecomment-1257883895
    unsafe {
        core::arch::asm! {"
                push rax
                push rcx
                push rdx
                push r8
                push r9
                push r10
                push r11

                // 1st arg (rdi): response
                // 2nd arg (rsi): request
                // 3rd arg (rdx): stack_frame
                lea rdx, [rsp + 7*8]
                call {}

                pop r11
                pop r10
                pop r9
                pop r8
                pop rdx
                pop rcx
                pop rax

                iretq
            ",
            sym syscall_handler_inner,
            options(noreturn)
        };
    }
}

extern "C" fn syscall_handler_inner(
    response: &mut Result<Response, Error>,
    request: &Request,
    stack_frame: &InterruptStackFrame,
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
