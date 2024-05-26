#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

use core::arch::asm;

use limine::{
    memory_map::EntryType,
    request::{FramebufferRequest, MemoryMapRequest},
    response::FramebufferResponse,
    smp::Cpu,
    BaseRevision,
};
use spin::Lazy;
use x86_64::registers::control::Cr3;

mod debugcon;
mod macros;

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
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
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

    kprintln!("Hello, World!");

    kprintln!("CPUs: {}", SMP_REQUEST.get_response().unwrap().cpus().len());

    kprintln!("Cr3: {:?}", Cr3::read());

    assert!(SMP_REQUEST.get_response().is_some());

    print_memory_map();

    start_cpus();

    if let Some(framebuffer) = FRAMEBUFFER_RESPONSE.framebuffers().next() {
        for i in 0..100_u64 {
            // Calculate the pixel offset using the framebuffer information we obtained above.
            // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
            let pixel_offset = i * framebuffer.pitch() + i * 4;

            // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
            unsafe { *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFFFFFFF };
        }
    }

    hcf();
}

extern "C" fn _start_core(cpu: &Cpu) -> ! {
    kprintln!("CPU{} started!", cpu.id);
    hcf();
}

fn start_cpus() {
    for cpu in SMP_RESPONSE.cpus() {
        if cpu.id == 0 {
            continue;
        }

        kprintln!("Starting CPU{}...", cpu.id);

        cpu.goto_address.write(_start_core);
    }
}

fn print_memory_map() {
    kprintln!("Memory map:");

    let mut reclaimable = 0;
    let mut usable = 0;

    for entry in MEMORY_MAP_RESPONSE.entries() {
        let base = entry.base;
        let length = entry.length;
        let end = base + length - 1;
        let entry_type = entry.entry_type;
        kprintln!(
            "{base:8x} - {end:8x} : {:?}",
            match entry_type {
                EntryType::USABLE => "Usable",
                EntryType::RESERVED => "Reserved",
                EntryType::ACPI_RECLAIMABLE => "ACPI Reclaimable",
                EntryType::ACPI_NVS => "ACPI NVS",
                EntryType::BAD_MEMORY => "Bad Memory",
                EntryType::BOOTLOADER_RECLAIMABLE => "Bootloader Reclaimable",
                EntryType::KERNEL_AND_MODULES => "Kernel and Modules",
                EntryType::FRAMEBUFFER => "Framebuffer",
                _ => "Unknown",
            }
        );

        match entry_type {
            EntryType::USABLE => usable += length,
            EntryType::ACPI_RECLAIMABLE => reclaimable += length,
            EntryType::BOOTLOADER_RECLAIMABLE => reclaimable += length,
            _ => {}
        }
    }

    kprintln!("Usable memory: {} KiB", usable / 1024);
    kprintln!("Reclaimable memory: {} KiB", reclaimable / 1024);
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
