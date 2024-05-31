#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use core::fmt::Write;

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

#[used]
#[link_section = ".requests"]
static SMP_REQUEST: limine::SmpRequest = limine::SmpRequest::new(limine::SmpFlags::X2APIC);

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
    assert!(SMP_REQUEST.response.get().is_some());

    structures::gdt::init();
    structures::idt::init();

    println!("Hello, World!");
    CONSOLE.lock().write_str("Hello, World!\n");

    println!("{:?}", SMP_REQUEST.response.get().unwrap());
    let _ = CONSOLE
        .lock()
        .write_fmt(format_args!("{:#?}\n", SMP_REQUEST.response.get().unwrap()));

    instructions::breakpoint();

    println!("We're back!");
    CONSOLE.lock().write_str("We're back!\n");

    SMP_REQUEST
        .response
        .get()
        .unwrap()
        .cpus()
        .iter()
        .skip(1)
        .for_each(|info| {
            println!("Starting CPU {}", info.processor_id);
            let _ = CONSOLE
                .lock()
                .write_fmt(format_args!("Starting CPU {}\n", info.processor_id));
            info.goto_address.write(smp_start);
        });

    println!("{:?}", RFlags::read());
    enable_interrupts();

    loop {
        instructions::hlt();
    }
}

extern "C" fn smp_start(info: &limine::SmpInfo) -> ! {
    let _ = CONSOLE
        .lock()
        .write_fmt(format_args!("Hello from CPU {}\n", info.processor_id));

    loop {
        instructions::hlt();
    }
}

#[cfg_attr(not(test), panic_handler)]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
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
