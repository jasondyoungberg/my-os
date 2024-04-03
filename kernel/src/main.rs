#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![warn(unused_unsafe)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)] // TODO: remove this later

use graphics::Drawable;

use crate::{
    bench::benchmark,
    graphics::color::{self, Color},
};

mod bench;
mod debugcon;
mod display;
mod font;
mod gdt;
mod graphics;
mod idt;
mod tss;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 0x20_0000; // 2 MiB
    config
};

bootloader_api::entry_point!(main, config = &CONFIG);

fn main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    init(boot_info);
    println!("Hello, world!");

    let mut display = display::DISPLAY.get().unwrap().lock();

    benchmark("clear", || {
        display.clear(&color::BLACK);
    });

    benchmark("rainbow", || {
        let size = display.size();
        for y in 0..size.1 {
            for x in 0..size.0 {
                display
                    .set_pixel((x, y), &Color::new(x as u8, y as u8, 128))
                    .unwrap()
            }
        }
    });

    benchmark("hello", || {
        for (i, c) in "Hello, world!".chars().enumerate() {
            let x = i * 8;
            let y = 0;
            display
                .blit((x, y), font::get_char_icon(c).unwrap())
                .unwrap();
        }
    });

    halt()
}

fn init(boot_info: &'static mut bootloader_api::BootInfo) {
    idt::load();
    gdt::load();

    display::init(boot_info.framebuffer.take().expect("no framebuffer"));
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
