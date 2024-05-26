#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

use core::arch::asm;

use limine::{
    request::{FramebufferRequest, MemoryMapRequest},
    response::FramebufferResponse,
    BaseRevision,
};
use spin::Lazy;

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

    hcf();
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
