#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use drivers::{
    console::{self, Console},
    display,
};
use instructions::enable_interrupts;
use registers::rflags::RFlags;
use spin::{Lazy, Mutex};

mod address;
mod allocation;
mod drivers;
mod instructions;
mod interrupts;
mod limine;
mod mapping;
mod registers;
mod structures;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: limine::MemoryMapRequest = limine::MemoryMapRequest::new();

static CONSOLE: Lazy<Mutex<Console>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_REQUEST
        .response
        .get()
        .unwrap()
        .framebuffers()
        .next()
        .unwrap();
    let console = Console::new(display::Display::new(framebuffer));
    Mutex::new(console)
});

#[cfg_attr(not(test), no_mangle)]
extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    assert!(FRAMEBUFFER_REQUEST.response.get().is_some());
    assert!(MEMORY_MAP_REQUEST.response.get().is_some());

    structures::gdt::init();
    structures::idt::init();

    println!("Hello, World!");
    CONSOLE.lock().write_str("Hello, World!\n");

    instructions::breakpoint();

    println!("We're back!");
    CONSOLE.lock().write_str("We're back!\n");

    println!("{:?}", RFlags::read());
    enable_interrupts();

    loop {
        instructions::hlt();
    }
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
