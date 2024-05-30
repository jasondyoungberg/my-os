#![no_std]
#![cfg_attr(not(test), no_main)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use drivers::{console, display};
use limine::FramebufferRequest;

mod drivers;
mod heap;
mod instructions;
mod limine;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new();

#[cfg_attr(not(test), no_mangle)]
extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    assert!(FRAMEBUFFER_REQUEST.response.get().is_some());

    println!("Hello, World!");

    let framebuffer = FRAMEBUFFER_REQUEST
        .response
        .get()
        .unwrap()
        .framebuffers()
        .next();

    let display = display::Display::new(framebuffer.unwrap());
    let mut console = console::Console::new(display);

    console.write_str("Hello, World!");

    panic!("End of main");
}

#[cfg_attr(not(test), panic_handler)]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    instructions::disable_interrupts();
    println!("{}", info);

    if let Some(framebuffers) = FRAMEBUFFER_REQUEST.response.get() {
        for framebuffer in framebuffers.framebuffers() {
            let display = display::Display::new(framebuffer);
            let mut console = console::Console::new(display);

            let _ = console.write_fmt(format_args!("{}\n", info));
        }
    }

    loop {
        instructions::hlt()
    }
}
