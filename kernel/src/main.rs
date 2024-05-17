#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
//
#![allow(dead_code)] // TODO: remove this later

use core::time::Duration;

use stdlib::prelude::*;

use crate::task::{Executor, Task};

extern crate alloc;

mod allocator;
mod bench;
mod debugcon;
mod disk;
mod display;
mod font;
mod graphics;
mod interrupts;
mod keyboard;
mod pretty;
mod task;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 0x20_0000; // 2 MiB
    config
};

#[cfg(not(test))]
bootloader_api::entry_point!(start, config = &CONFIG);

fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    interrupts::init();
    keyboard::init();
    display::init(boot_info.framebuffer.take().expect("no framebuffer"));

    info!("Hello, world!");
    syscall::print("Hello, syscall!").unwrap();

    let mut executor = Executor::new();
    executor.spawn(Task::new(print_keypresses()));
    executor.spawn(Task::new(print_bootsector()));
    executor.spawn(Task::new(count()));
    executor.run()
}

async fn print_bootsector() {
    let mut disk = unsafe { disk::AtaDisk::new(0x1F0) };
    let mut buffer = [0u8; 512];
    disk.read_sectors(0, 1, &mut buffer).await;
    println!("{}", pretty::Hexdump(&buffer));
}

async fn count() {
    use task::delay;

    for i in 0.. {
        if i % 2 == 0 {
            println!("tick {i}");
        } else {
            println!("tock {i}");
        }

        delay(Duration::from_secs(1)).await;
    }
}

async fn print_keypresses() {
    use futures::StreamExt;
    use pc_keyboard::{layouts::Us104Key, HandleControl, Keyboard, ScancodeSet1};

    let mut scancodes = keyboard::ScancodeStream::new();
    let mut keyboard: Keyboard<Us104Key, ScancodeSet1> =
        Keyboard::new(ScancodeSet1::new(), Us104Key, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Some(key_event) = keyboard.add_byte(scancode).unwrap() {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    pc_keyboard::DecodedKey::Unicode(character) => print!("{}", character),
                    pc_keyboard::DecodedKey::RawKey(key) => print!("[{:?}]", key),
                }
            }
        }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);

    loop {
        x86_64::instructions::hlt()
    }
}
