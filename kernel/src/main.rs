#![no_std]
#![no_main]

use core::arch::asm;

use limine::{
    memory_map::EntryType,
    request::{FramebufferRequest, MemoryMapRequest},
    BaseRevision,
};

mod debugcon;
mod macros;

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());

    kprintln!("Hello, World!");

    // Print the memory map.
    print_memory_map();

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..100_u64 {
                // Calculate the pixel offset using the framebuffer information we obtained above.
                // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
                let pixel_offset = i * framebuffer.pitch() + i * 4;

                // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
                *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFFFFFFF;
            }
        }
    }

    hcf();
}

fn print_memory_map() {
    kprintln!("Memory map:");

    let mut reclaimable = 0;
    let mut usable = 0;

    if let Some(memory_map_response) = MEMORY_MAP_REQUEST.get_response() {
        for entry in memory_map_response.entries() {
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

            match EntryType::from(entry_type) {
                EntryType::USABLE => usable += length,
                EntryType::ACPI_RECLAIMABLE => reclaimable += length,
                EntryType::BOOTLOADER_RECLAIMABLE => reclaimable += length,
                _ => {}
            }
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
