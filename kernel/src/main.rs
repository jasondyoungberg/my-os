#![no_std]
#![no_main]

mod drivers;
mod instructions;
mod limine;

#[used]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    assert!(FRAMEBUFFER_REQUEST.response.get().is_some());

    drivers::debucon::print("Hello, World!\n");

    let framebuffer = FRAMEBUFFER_REQUEST
        .response
        .get()
        .unwrap()
        .framebuffers()
        .next()
        .unwrap();

    for i in 0..100_u64 {
        // Calculate the pixel offset using the framebuffer information we obtained above.
        // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
        let pixel_offset = i * framebuffer.pitch() + i * 4;

        // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
        *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFFFFFFF;
    }

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
