#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
//
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

extern crate alloc;

use core::panic::PanicInfo;

use alloc::boxed::Box;
use limine::{
    request::{FramebufferRequest, HhdmRequest, MemoryMapRequest},
    response::FramebufferResponse,
    smp::Cpu,
    BaseRevision,
};
use spin::Lazy;
use x86_64::{
    instructions::{hlt, interrupts, port::PortWriteOnly},
    registers::model_specific::{GsBase, KernelGsBase},
    VirtAddr,
};

use crate::coredata::CoreData;

mod coredata;
mod debugcon;
mod heap;
mod idt;
mod lapic;
mod logger;
mod macros;
mod memory;
mod pics;
mod process;

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

static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
static HHDM_RESPONSE: Lazy<&limine::response::HhdmResponse> =
    Lazy::new(|| HHDM_REQUEST.get_response().unwrap());

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
    assert!(HHDM_REQUEST.get_response().is_some(), "HHDM request failed");

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

    // Initialize CPU
    if cpu.id == 0 {
        pics::init();
    }

    idt::IDT.load();
    let lapic = lapic::init();

    // Setup core data
    let core_data = Box::pin(CoreData {
        id: cpu.id,
        lapic: Box::new(lapic),
    });
    let core_data_ptr = &*core_data as *const _ as *const ();
    let core_data_addr = VirtAddr::from_ptr(core_data_ptr);

    GsBase::write(core_data_addr);
    KernelGsBase::write(core_data_addr);

    interrupts::enable();

    log::info!("Ready!");

    loop {
        hlt();
    }
}

#[panic_handler]
fn rust_panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    shutdown_emu();
}

fn shutdown_emu() -> ! {
    unsafe { PortWriteOnly::<u16>::new(0x604).write(0x2000) };

    interrupts::disable();
    loop {
        hlt();
    }
}
