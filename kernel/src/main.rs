#![no_std]
#![no_main]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity
)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(
    unused_unsafe,
    clippy::missing_safety_doc,
    clippy::multiple_unsafe_ops_per_block,
    clippy::undocumented_unsafe_blocks
)]
#![allow(clippy::cast_possible_truncation)] // Only x86_64 is supported

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This should only be compiled for x86_64 targets.");

mod debug;

use limine::request::FramebufferRequest;
use limine::BaseRevision;
use x86_64::instructions::{hlt, interrupts};

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

/// # Safety
/// This function should only be called once.
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());

    println!("Hello, World!");

    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    hcf();
}

fn hcf() -> ! {
    interrupts::disable();
    loop {
        hlt();
    }
}
