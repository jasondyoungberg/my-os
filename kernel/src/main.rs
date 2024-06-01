#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::{instructions::enable_interrupts, registers::ApicBase};

extern crate alloc;

mod address;
mod allocation;
mod drivers;
mod instructions;
mod interrupts;
mod limine;
mod macros;
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
static SMP_REQUEST: limine::SmpRequest = limine::SmpRequest::new(limine::SmpFlags::empty());

#[used]
#[link_section = ".requests"]
static STACK_SIZE_REQUEST: limine::StackSizeRequest = limine::StackSizeRequest::new(1024 * 1024);

#[used]
#[link_section = ".requests"]
static MODULE_REQUEST: limine::ModuleRequest = limine::ModuleRequest::new(&[]);

#[used]
#[link_section = ".requests"]
static HHDP_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new();

#[cfg_attr(not(test), no_mangle)]
extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    assert!(FRAMEBUFFER_REQUEST.response.get().is_some());
    assert!(MEMORY_MAP_REQUEST.response.get().is_some());
    assert!(SMP_REQUEST.response.get().is_some());
    assert!(STACK_SIZE_REQUEST.response.get().is_some());
    assert!(MODULE_REQUEST.response.get().is_some());
    assert!(HHDP_REQUEST.response.get().is_some());

    structures::gdt::init();
    structures::idt::init();

    println!("Hello, World!");

    println!("{:?}", SMP_REQUEST.response.get().unwrap());

    for file in MODULE_REQUEST.response.get().unwrap().modules() {
        println!("Module: {}", file.path());
    }

    SMP_REQUEST
        .response
        .get()
        .unwrap()
        .cpus()
        .iter()
        .skip(1)
        .for_each(|info| {
            println!("Starting CPU {}", info.processor_id);
            info.goto_address.write(smp_start);
        });

    // enable_interrupts();

    loop {
        instructions::hlt();
    }
}

extern "C" fn smp_start(info: &limine::SmpInfo) -> ! {
    println!("Hello from CPU {}", info.processor_id);

    ApicBase::enable();

    enable_interrupts();

    loop {
        instructions::hlt();
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
