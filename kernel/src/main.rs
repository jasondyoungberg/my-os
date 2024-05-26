#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

extern crate alloc;

use limine::{
    request::{FramebufferRequest, MemoryMapRequest},
    response::FramebufferResponse,
    smp::Cpu,
    BaseRevision,
};
use spin::Lazy;
use x86_64::instructions::{hlt, interrupts};

mod debugcon;
mod heap;
mod idt;
mod logger;
mod macros;
mod pics;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
static FRAMEBUFFER_RESPONSE: Lazy<&FramebufferResponse> =
    Lazy::new(|| FRAMEBUFFER_REQUEST.get_response().unwrap());

#[used]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
static MEMORY_MAP_RESPONSE: Lazy<&limine::response::MemoryMapResponse> =
    Lazy::new(|| MEMORY_MAP_REQUEST.get_response().unwrap());

#[used]
static SMP_REQUEST: limine::request::SmpRequest = limine::request::SmpRequest::new();
static SMP_RESPONSE: Lazy<&limine::response::SmpResponse> =
    Lazy::new(|| SMP_REQUEST.get_response().unwrap());

#[no_mangle]
extern "C" fn _start() -> ! {
    // All limine requests must also be referenced in a called function,
    // otherwise they may be removed by the linker.
    assert!(BASE_REVISION.is_supported(), "Unsupported base revision");
    assert!(
        FRAMEBUFFER_REQUEST.get_response().is_some(),
        "Framebuffer request failed"
    );
    assert!(
        MEMORY_MAP_REQUEST.get_response().is_some(),
        "Memory map request failed"
    );
    assert!(SMP_REQUEST.get_response().is_some(), "SMP request failed");

    logger::init();

    for cpu in SMP_RESPONSE.cpus() {
        if cpu.id != 0 {
            log::info!("Starting CPU{}", cpu.id);
            cpu.goto_address.write(_start_cpu);
        }
    }

    _start_cpu(SMP_RESPONSE.cpus()[0]);
}

extern "C" fn _start_cpu(cpu: &Cpu) -> ! {
    log::info!("CPU{} started", cpu.id);

    // Initialize once
    if cpu.id == 0 {
        pics::init();
    }

    // Initialize per CPU
    idt::IDT.load();

    interrupts::enable();

    loop {
        hlt();
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);
    loop {
        hlt();
    }
}
