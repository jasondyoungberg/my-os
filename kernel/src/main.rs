#![no_std]
#![no_main]
#![warn(unused_unsafe)]
#![deny(unsafe_op_in_unsafe_fn)]

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(main, config = &CONFIG);

fn main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    cpu::halt()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cpu::halt()
}
