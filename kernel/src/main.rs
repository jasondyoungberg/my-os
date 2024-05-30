#![no_std]
#![cfg_attr(not(test), no_main)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use alloc::boxed::Box;
use drivers::display;

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

    loop {
        for z in 0..255 {
            for y in 0..display.height {
                for x in 0..display.width {
                    let r = (x % 255) as u32;
                    let g = (y % 255) as u32;
                    let b = z;
                    let color = (r << 16) | (g << 8) | b;
                    display.set_pixel(x, y, color);
                }
            }
        }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    instructions::disable_interrupts();
    println!("{}", info);
    loop {
        instructions::hlt()
    }
}
