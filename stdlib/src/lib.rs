#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

pub mod syscall;

/// Write a message to stdout.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
pub fn write(msg: &str) -> Result<(), u64> {
    unsafe {
        syscall::syscall2(
            syscall::SyscallId::Write,
            msg.as_ptr() as u64,
            msg.len() as u64,
        )
    }
    .map(|_| ())
}
