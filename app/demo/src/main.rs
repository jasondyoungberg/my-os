#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, World!";

    stdlib::write(msg).unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let _ = stdlib::write("Panic!\n");

    #[allow(clippy::empty_loop)]
    loop {}
}
