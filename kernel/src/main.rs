#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
#![warn(unused_unsafe)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)] // TODO: remove this later

extern crate alloc;

mod allocator;
mod bench;
mod debugcon;
mod disk;
mod display;
mod font;
mod gdt;
mod graphics;
mod idt;
mod pretty;
mod tss;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 0x20_0000; // 2 MiB
    config
};

#[cfg(not(test))]
bootloader_api::entry_point!(start, config = &CONFIG);

fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    init(boot_info);

    main();
    halt()
}

fn main() {
    println!("Hello, world!");
}

fn init(boot_info: &'static mut bootloader_api::BootInfo) {
    idt::load();
    gdt::load();

    display::init(boot_info.framebuffer.take().expect("no framebuffer"));
}

#[cfg_attr(not(test), panic_handler)]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    println!("{}", panic_info);
    halt()
}

fn halt() -> ! {
    loop {
        x86_64::instructions::hlt()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn trivial() {}

    #[test]
    #[should_panic]
    fn panic() {
        panic!()
    }
}
