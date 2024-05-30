#![no_std]
#![no_main]

mod drivers;
mod instructions;
mod limine;

#[used]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    drivers::debucon::print("Hello, World!\n");

    loop {
        instructions::hlt()
    }
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        instructions::hlt()
    }
}
