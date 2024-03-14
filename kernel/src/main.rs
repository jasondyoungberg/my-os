#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![warn(unused_unsafe)]
#![deny(unsafe_op_in_unsafe_fn)]

mod debugcon;
mod idt;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(main, config = &CONFIG);

fn main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    idt::init();
    println!("Hello, world!");
    halt()
}

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    println!("{}", panic_info);
    halt()
}

fn halt() -> ! {
    loop {
        x86_64::instructions::hlt()
    }
}
