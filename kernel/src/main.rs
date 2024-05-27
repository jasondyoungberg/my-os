#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
//
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

extern crate alloc;

use core::panic::PanicInfo;

use limine::{
    request::{FramebufferRequest, HhdmRequest, MemoryMapRequest, StackSizeRequest},
    response::FramebufferResponse,
    smp::Cpu,
    BaseRevision,
};
use spin::{Lazy, Mutex};
use x86_64::instructions::{hlt, interrupts, port::PortWriteOnly};

use crate::{
    gsdata::{CpuId, KernelData},
    ministack::create_ministack,
    process::{Manager, MANAGER},
};

mod debugcon;
mod display;
mod exception;
mod gdt;
mod gsdata;
mod heap;
mod idt;
mod lapic;
mod logger;
mod macros;
mod memory;
mod ministack;
mod pics;
mod process;
mod syscall;

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

static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(1024 * 1024); // 1 MiB
static STACK_SIZE_RESPONSE: Lazy<&limine::response::StackSizeResponse> =
    Lazy::new(|| STACK_SIZE_REQUEST.get_response().unwrap());

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
    assert!(
        STACK_SIZE_REQUEST.get_response().is_some(),
        "Stack size request failed"
    );

    logger::init();

    MANAGER.call_once(|| Mutex::new(Manager::init()));
    pics::init();

    for cpu in SMP_RESPONSE.cpus() {
        if cpu.id != 0 {
            log::info!("Starting CPU{}", cpu.id);
            cpu.goto_address.write(_start_cpu);
        }
    }

    _start_cpu(SMP_RESPONSE.cpus()[0]);
}

extern "C" fn _start_cpu(cpu: &Cpu) -> ! {
    let cpuid = CpuId::new(cpu.id);

    log::info!("{} started", cpuid);

    gdt::init(cpuid);
    idt::IDT.load();
    let lapic = lapic::init();
    syscall::init();

    log::info!("{} joining kernel", cpuid);
    let active_thread = MANAGER.get().unwrap().lock().join_kernel();

    // Setup core data
    let kernel_gs_data = KernelData::new(
        cpuid,
        create_ministack(64 * 1024), // 64 KiB
        lapic,
        active_thread,
    );

    kernel_gs_data.as_ref().save_kernel_gsbase();

    interrupts::enable();

    log::info!("{cpuid} Ready!");

    if cpu.id == 0 {
        let mut manager = MANAGER.get().unwrap().lock();
        manager.spawn(include_bytes!("../app/test1"));
    }

    loop {
        log::debug!("{} {:?}", cpuid, active_thread);
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
