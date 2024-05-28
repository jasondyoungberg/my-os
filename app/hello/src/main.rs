#![no_std]
#![no_main]

// #[macro_use]
// extern crate stdlib;

// entry!(main);
// fn main() {
//     println!("Hello, world!");
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, world! I'm (barely) a rust program!\n";
    let msg_ptr = msg.as_ptr() as u64;
    let msg_len = msg.len();

    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 1 => _,
            in("rdi") 1,
            in("rsi") msg_ptr,
            in("rdx") msg_len,
            out("rcx") _,
            out("r11") _,

            options(nostack, preserves_flags)
        )
    }
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") 60 => _,
            in("rdi") 0,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        )
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
