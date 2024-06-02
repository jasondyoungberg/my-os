#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use drivers::lapic;
use gsdata::GsData;
use process::Process;
use spin::Lazy;
use x86_64::VirtAddr;

extern crate alloc;

mod allocation;
mod drivers;
mod gdt;
mod gsdata;
mod idt;
mod macros;
mod mapping;
mod process;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest =
    limine::request::FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: limine::request::MemoryMapRequest =
    limine::request::MemoryMapRequest::new();

#[used]
#[link_section = ".requests"]
static SMP_REQUEST: limine::request::SmpRequest = limine::request::SmpRequest::new();

#[used]
#[link_section = ".requests"]
static STACK_SIZE_REQUEST: limine::request::StackSizeRequest =
    limine::request::StackSizeRequest::new().with_size(1024 * 1024);

#[used]
#[link_section = ".requests"]
static MODULE_REQUEST: limine::request::ModuleRequest = limine::request::ModuleRequest::new();

#[used]
#[link_section = ".requests"]
static HHDP_REQUEST: limine::request::HhdmRequest = limine::request::HhdmRequest::new();

static FRAMEBUFFER_RESPONSE: Lazy<&limine::response::FramebufferResponse> =
    Lazy::new(|| FRAMEBUFFER_REQUEST.get_response().unwrap());
static MEMORY_MAP_RESPONSE: Lazy<&limine::response::MemoryMapResponse> =
    Lazy::new(|| MEMORY_MAP_REQUEST.get_response().unwrap());
static SMP_RESPONSE: Lazy<&limine::response::SmpResponse> =
    Lazy::new(|| SMP_REQUEST.get_response().unwrap());
static STACK_SIZE_RESPONSE: Lazy<&limine::response::StackSizeResponse> =
    Lazy::new(|| STACK_SIZE_REQUEST.get_response().unwrap());
static MODULE_RESPONSE: Lazy<&limine::response::ModuleResponse> =
    Lazy::new(|| MODULE_REQUEST.get_response().unwrap());
static HHDP_RESPONSE: Lazy<&limine::response::HhdmResponse> =
    Lazy::new(|| HHDP_REQUEST.get_response().unwrap());

#[no_mangle]
extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    assert!(FRAMEBUFFER_REQUEST.get_response().is_some());
    assert!(MEMORY_MAP_REQUEST.get_response().is_some());
    assert!(SMP_REQUEST.get_response().is_some());
    assert!(STACK_SIZE_REQUEST.get_response().is_some());
    assert!(MODULE_REQUEST.get_response().is_some());
    assert!(HHDP_REQUEST.get_response().is_some());

    println!("Starting CPUs");
    let bsp_lapic_id = SMP_RESPONSE.bsp_lapic_id();
    SMP_RESPONSE
        .cpus()
        .iter()
        .filter(|cpu| cpu.lapic_id != bsp_lapic_id)
        .for_each(|cpu| cpu.goto_address.write(smp_start));

    let bsp_cpu = SMP_RESPONSE
        .cpus()
        .iter()
        .find(|cpu| cpu.lapic_id == bsp_lapic_id)
        .unwrap();

    smp_start(bsp_cpu)
}

extern "C" fn smp_start(this_cpu: &limine::smp::Cpu) -> ! {
    let bsp_lapic_id = SMP_RESPONSE.bsp_lapic_id();

    let cpuid = SMP_RESPONSE
        .cpus()
        .iter()
        .filter(|cpu| cpu.lapic_id != bsp_lapic_id)
        .enumerate()
        .find(|(_, cpu)| cpu.lapic_id == this_cpu.lapic_id);

    let cpuid = cpuid.map(|(i, _)| i + 1).unwrap_or(0);

    println!("Hello from CPU {}", cpuid);

    gdt::init(cpuid);
    idt::init();

    let mut lapic = lapic::LocalApic::new();
    lapic.init();
    GsData::init(VirtAddr::zero(), cpuid, lapic);

    if cpuid == 0 {
        Process::create_root(root_process);
    }

    x86_64::instructions::interrupts::enable();

    loop {
        println!("CPU {} is doing nothing", cpuid);
        x86_64::instructions::hlt();
    }
}

extern "C" fn root_process() -> ! {
    println!("Hello from root");

    loop {
        println!("Root is doing nothing");
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    x86_64::instructions::interrupts::disable();

    unsafe { macros::force_print(format_args!("{}", info)) };

    loop {
        x86_64::instructions::hlt();
    }
}
