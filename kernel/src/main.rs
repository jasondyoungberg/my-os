#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate klib;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This should only be compiled for x86_64 targets.");

pub mod alloc_frame;
pub mod requests;

use x86_64::instructions::{hlt, interrupts};

/// # Safety
/// This function should only be called by the bootloader.
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    requests::verify();

    kprintln!("Hello, World!");

    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);
    hcf();
}

fn hcf() -> ! {
    interrupts::disable();
    loop {
        hlt();
    }
}
