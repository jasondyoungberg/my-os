#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use address::VirtAddr;
use drivers::lapic;
use gsdata::GsData;

use crate::instructions::enable_interrupts;

extern crate alloc;

mod address;
mod allocation;
mod drivers;
mod gsdata;
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

#[no_mangle]
extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    assert!(FRAMEBUFFER_REQUEST.response.get().is_some());
    assert!(MEMORY_MAP_REQUEST.response.get().is_some());
    assert!(SMP_REQUEST.response.get().is_some());
    assert!(STACK_SIZE_REQUEST.response.get().is_some());
    assert!(MODULE_REQUEST.response.get().is_some());
    assert!(HHDP_REQUEST.response.get().is_some());

    println!("Starting CPUs");
    SMP_REQUEST
        .response
        .get()
        .unwrap()
        .cpus()
        .iter()
        .skip(1)
        .for_each(|info| {
            info.goto_address.write(smp_start);
        });

    smp_start(SMP_REQUEST.response.get().unwrap().cpus()[0])
}

extern "C" fn smp_start(info: &limine::SmpInfo) -> ! {
    println!("Hello from CPU {}", info.processor_id);

    structures::gdt::init();
    structures::idt::init();

    let lapic = lapic::LocalApic::new();
    lapic.init();
    GsData::init(VirtAddr::null(), info.processor_id, lapic);

    enable_interrupts();

    loop {
        instructions::hlt();
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    instructions::disable_interrupts();

    unsafe { macros::force_print(format_args!("{}", info)) };

    loop {
        instructions::hlt()
    }
}
