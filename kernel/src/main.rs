#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(asm_const)]
// TODO: remove these later
#![allow(dead_code)]

use core::time::Duration;

use bootloader_api::config::Mapping::FixedAddress;
use memory::PHYSICAL_MEMORY_OFFSET;
use x86_64::instructions::interrupts::int3;

extern crate alloc;

mod bench;
mod debugcon;
mod disk;
mod display;
mod font;
mod graphics;
mod interrupts;
mod keyboard;
mod macros;
mod memory;
mod pretty;
mod syscall;
mod task;
mod threading;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    // config.mappings.kernel_stack = FixedAddress(memory::KERNEL_STACK);
    config.mappings.physical_memory = Some(FixedAddress(PHYSICAL_MEMORY_OFFSET));
    config.mappings.dynamic_range_start = Some(memory::KERNEL_DYNAMIC);
    config.mappings.dynamic_range_end = Some(memory::KERNEL_DYNAMIC_END);

    config.kernel_stack_size = 0x10_0000; // 1 MiB
    config
};

#[cfg(not(test))]
bootloader_api::entry_point!(start, config = &CONFIG);

#[allow(clippy::needless_pass_by_ref_mut)]
fn start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    assert_eq!(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("physical memory is not mapped"),
        PHYSICAL_MEMORY_OFFSET,
        "physical memory is not mapped to the correct address"
    );

    debugcon::init();

    interrupts::init();
    syscall::init();
    memory::init(&boot_info.memory_regions);
    // memory::print();

    // keyboard::init();
    // display::init(boot_info.framebuffer.take().expect("no framebuffer"));

    kprintln!("Hello, world!");
    log::info!("Hello, world!");

    // syscall::print("Hello, syscall!").unwrap();

    // println!(
    //     "{}",
    //     Hexdump(unsafe { slice::from_raw_parts(0x13d000 as *const u8, 65536) })
    // );

    let mut manager = threading::manager::MANAGER.lock();

    // memory::print();

    // manager.spawn(include_bytes!("../../test-app/test_a"));
    manager.spawn(include_bytes!("../../test-app/test_b"));

    drop(manager);

    kprint!("Kernel 1");
    int3();
    kprint!("Kernel 2");
    int3();
    kprint!("Kernel 3");

    exit_qemu();

    // error!("This is an error message");
    // warn!("This is a warning message");
    // info!("This is an info message");
    // debug!("This is a debug message");
    // trace!("This is a trace message");

    // let mut executor = Executor::new();
    // executor.spawn(Task::new(print_keypresses()));
    // executor.spawn(Task::new(print_bootsector()));
    // executor.spawn(Task::new(count()));
    // executor.run()
}

async fn print_bootsector() {
    let mut disk = unsafe { disk::AtaDisk::new(0x1F0) };
    let mut buffer = [0u8; 512];
    disk.read_sectors(0, 1, &mut buffer).await;
    kprintln!("{}", pretty::Hexdump(&buffer));
}

async fn count() {
    use task::delay;

    for i in 0.. {
        if i % 2 == 0 {
            kprintln!("tick {i}");
        } else {
            kprintln!("tock {i}");
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
                    pc_keyboard::DecodedKey::Unicode(character) => kprint!("{}", character),
                    pc_keyboard::DecodedKey::RawKey(key) => kprint!("[{:?}]", key),
                }
            }
        }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", panic_info);

    exit_qemu()
}

fn exit_qemu() -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port: Port<u8> = Port::new(0xf4);
        port.write(0);
    }

    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt()
    }
}
