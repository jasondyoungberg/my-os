#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    unused_unsafe,
    clippy::missing_safety_doc,
    clippy::multiple_unsafe_ops_per_block,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used
)]
#![allow(
    clippy::cast_possible_truncation, // Only x86_64 is supported
    clippy::missing_panics_doc // This crate should never panic on recoverable errors
)]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This should only be compiled for x86_64 targets.");

pub mod debug;
pub mod frame_alloc;
pub mod requests;

use x86_64::instructions::{hlt, interrupts};

/// # Safety
/// This function should only be called by the bootloader.
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    requests::verify();

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
