#[derive(Debug)]
pub enum Request<'a> {
    Print(&'a str),
}

#[derive(Debug)]
pub enum Response {
    Print,
}

#[derive(Debug)]
pub enum Error {
    NoResponse,
    InvalidResponse(Response),
}

/// Send a syscall to the kernel.
///
/// # Errors
///
/// - `SyscallError::InvalidResponse`: The kernel returned an unexpected response
pub fn syscall(request: &Request) -> Result<Response, Error> {
    let mut result: Result<Response, Error> = Err(Error::NoResponse);

    let request_ptr = core::ptr::from_ref(request);
    let result_ptr = core::ptr::from_mut(&mut result);

    unsafe {
        use core::arch::asm;
        asm!(
            "int 0x80",
            in("rsi") request_ptr,
            in("rdx") result_ptr,
            options(preserves_flags)
        );
    }

    result
}

/// Output a message
///
/// # Errors
///
/// - `SyscallError::InvalidResponse`: The kernel returned an unexpected response
pub fn print(msg: &str) -> Result<(), Error> {
    let result = syscall(&Request::Print(msg))?;

    #[allow(unreachable_patterns)] // todo: remove this once we have more syscalls
    match result {
        Response::Print => Ok(()),
        x => Err(Error::InvalidResponse(x)),
    }
}
