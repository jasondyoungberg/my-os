#![no_std]
#![no_main]

mod printer;
mod syscall;

pub use printer::*;
pub use syscall::*;

#[macro_export]
macro_rules! entry {
    ($main:path) => {
        const _: fn() = $main;

        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            $main()
            $crate::exit(0)
        }
    };
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    // we don't call exit here because we want to recurse if it fails
    let _ = unsafe { syscall1(60, -1i64 as u64) };
    loop {}
}

/// Writes the given data to the given channel.
/// Returns the number of bytes written.
pub fn write(channel: u64, data: &[u8]) -> Result<usize, u64> {
    let addr = data.as_ptr() as u64;
    let len = data.len() as u64;
    match unsafe { syscall3(1, channel, addr, len) } {
        Ok(n) => Ok(n as usize),
        Err(e) => Err(e),
    }
}

/// Yields the current thread.
pub fn yeild() -> Result<(), u64> {
    match unsafe { syscall0(24) } {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Exits the program with the given exit code.
pub fn exit(code: i64) -> ! {
    let _ = unsafe { syscall1(60, code as u64) };
    panic!("exit failed");
}
