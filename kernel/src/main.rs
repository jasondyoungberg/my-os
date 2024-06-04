#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
//
#![allow(dead_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::slice;

use drivers::{
    display::{Color, Display},
    lapic,
};
use gdt::create_ministack;
use gsdata::GsData;
use process::Process;
use spin::Lazy;

extern crate alloc;

mod allocation;
mod drivers;
mod gdt;
mod gsdata;
mod idt;
mod logger;
mod macros;
mod mapping;
mod process;
mod syscall;

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
static HHDM_REQUEST: limine::request::HhdmRequest = limine::request::HhdmRequest::new();

static FRAMEBUFFER_RESPONSE: Lazy<&limine::response::FramebufferResponse> = Lazy::new(|| {
    FRAMEBUFFER_REQUEST
        .get_response()
        .expect("no framebuffer response")
});
static MEMORY_MAP_RESPONSE: Lazy<&limine::response::MemoryMapResponse> = Lazy::new(|| {
    MEMORY_MAP_REQUEST
        .get_response()
        .expect("no memory map response")
});
static SMP_RESPONSE: Lazy<&limine::response::SmpResponse> =
    Lazy::new(|| SMP_REQUEST.get_response().expect("no smp response"));
static STACK_SIZE_RESPONSE: Lazy<&limine::response::StackSizeResponse> = Lazy::new(|| {
    STACK_SIZE_REQUEST
        .get_response()
        .expect("no stack size response")
});
static MODULE_RESPONSE: Lazy<&limine::response::ModuleResponse> =
    Lazy::new(|| MODULE_REQUEST.get_response().expect("no module response"));
static HHDM_RESPONSE: Lazy<&limine::response::HhdmResponse> =
    Lazy::new(|| HHDM_REQUEST.get_response().expect("no hhdm response"));

fn load_file(name: &str) -> Option<&'static [u8]> {
    let file = MODULE_RESPONSE
        .modules()
        .iter()
        .find(|file| file.path() == name.as_bytes())?;

    let addr = file.addr();
    let size = file.size() as usize;
    let slice = unsafe { slice::from_raw_parts(addr, size) };
    Some(slice)
}

#[no_mangle]
extern "C" fn _start() -> ! {
    logger::init();
    log::debug!("{:?}\n", BASE_REVISION);
    log::debug!("{:?}\n", *FRAMEBUFFER_RESPONSE);
    log::debug!("{:?}\n", FRAMEBUFFER_RESPONSE.framebuffers().next());
    log::debug!("{:?}\n", *MEMORY_MAP_RESPONSE);
    log::debug!("{:?}\n", *SMP_RESPONSE);
    log::debug!("{:?}\n", *STACK_SIZE_RESPONSE);
    log::debug!("{:?}\n", *MODULE_RESPONSE);
    log::debug!("{:?}\n", *HHDM_RESPONSE);

    log::info!("Starting CPUs");
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

    log::info!("CPU {cpuid} is started");

    log::trace!("CPU {cpuid} initializing tss/gdt/idt");
    gdt::init(cpuid);
    idt::init();

    log::trace!("CPU {cpuid} initializing lapic");
    let mut lapic = lapic::LocalApic::new();
    lapic.init();
    GsData::init(create_ministack(), cpuid, lapic);

    log::trace!("CPU {cpuid} initializing syscall");
    syscall::init();

    if cpuid == 0 {
        log::trace!("spawning root process");
        Process::create_root(root_process);
    }

    log::info!("CPU {cpuid} is ready");
    x86_64::instructions::interrupts::enable();

    loop {
        x86_64::instructions::hlt();
    }
}

extern "C" fn root_process() -> ! {
    log::info!("root process started");

    let gsdata = unsafe { GsData::load_kernel().expect("root process gsdata is missing") };
    let process = gsdata
        .process
        .as_mut()
        .expect("root process gsdata.process is missing");

    if let Some(file) = load_file("/bin/hello") {
        process.create_user("hello", file);
    } else {
        log::warn!("failed to load /bin/hello");
    }

    if let Some(file) = load_file("/bin/loop") {
        process.create_user("echo", file);
    } else {
        log::warn!("failed to load /bin/loop");
    }

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    x86_64::instructions::interrupts::disable();

    log::error!("{}\n", info);

    loop {
        x86_64::instructions::hlt();
    }
}
