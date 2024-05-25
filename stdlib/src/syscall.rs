use core::arch::asm;

#[derive(Debug)]
#[repr(u64)]
pub enum SyscallId {
    Exit,
    Read,
    Write,
}

impl TryFrom<u64> for SyscallId {
    type Error = u64;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Exit),
            1 => Ok(Self::Read),
            2 => Ok(Self::Write),
            _ => Err(value),
        }
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall0(n: SyscallId) -> Result<u64, u64> {
    let result: u64;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as u64 => result,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r15") _,
            options(nostack, preserves_flags)
        );
    }

    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall1(n: SyscallId, arg1: u64) -> Result<u64, u64> {
    let result: u64;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as u64 => result,
            in("rdi") arg1,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r15") _,
            options(nostack, preserves_flags)
        );
    }

    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall2(n: SyscallId, arg1: u64, arg2: u64) -> Result<u64, u64> {
    let result: u64;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as u64 => result,
            in("rdi") arg1,
            in("rsi") arg2,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r15") _,
            options(nostack, preserves_flags)
        );
    }

    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall3(n: SyscallId, arg1: u64, arg2: u64, arg3: u64) -> Result<u64, u64> {
    let result: u64;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as u64 => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r15") _,
            options(nostack, preserves_flags)
        );
    }

    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall4(
    n: SyscallId,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
) -> Result<u64, u64> {
    let result: u64;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as u64 => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r15") _,
            options(nostack, preserves_flags)
        );
    }

    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall5(
    n: SyscallId,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> Result<u64, u64> {
    let result: u64;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as u64 => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8") arg5,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r15") _,
            options(nostack, preserves_flags)
        );
    }

    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}

/// Issues a raw system call.
///
/// # Errors
/// If the system call returns an error, this function will return the error code.
///
/// # Safety
/// This function is unsafe because it performs a raw system call.
pub unsafe fn syscall6(
    n: SyscallId,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> Result<u64, u64> {
    let result: u64;

    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as u64 => result,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            in("r8") arg5,
            in("r9") arg6,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r15") _,
            options(nostack, preserves_flags)
        );
    }

    if result >= 0x8000_0000_0000_0000 {
        Err(result & 0x7fff_ffff_ffff_ffff)
    } else {
        Ok(result)
    }
}