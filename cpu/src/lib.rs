#![no_std]
#![warn(unused_unsafe)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::arch::asm;

pub mod io;

/// Halts the CPU
pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
