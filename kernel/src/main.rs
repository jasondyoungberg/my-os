#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use drivers::lapic;
use gsdata::GsData;
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
    SMP_REQUEST
        .get_response()
        .unwrap()
        .cpus()
        .iter()
        .skip(1)
        .for_each(|info| {
            info.goto_address.write(smp_start);
        });

    smp_start(SMP_REQUEST.get_response().unwrap().cpus()[0])
}

extern "C" fn smp_start(cpu: &limine::smp::Cpu) -> ! {
    println!("Hello from CPU {}", cpu.id);

    gdt::init(cpu.id);
    idt::init();

    let mut lapic = lapic::LocalApic::new();
    lapic.init();
    GsData::init(VirtAddr::zero(), cpu.id, lapic);

    x86_64::instructions::interrupts::enable();

    loop {
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
