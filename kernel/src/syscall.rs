use x86_64::{
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask, Star},
        rflags::RFlags,
    },
    VirtAddr,
};

use crate::{gdt::GDT, print, process::Registers, wrap};

pub fn init() {
    LStar::write(VirtAddr::new(handle_syscall as usize as u64));
    Star::write(
        GDT.user_code,
        GDT.user_data,
        GDT.kernel_code,
        GDT.kernel_data,
    )
    .unwrap();
    SFMask::write(RFlags::INTERRUPT_FLAG);

    let efer = Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS;

    unsafe { Efer::write(efer) };
}

wrap!(syscall, handle_syscall_inner => handle_syscall);

extern "C" fn handle_syscall_inner(registers: &mut Registers) {
    let num = registers.rax;
    let arg1 = registers.rdi;
    let arg2 = registers.rsi;
    let arg3 = registers.rdx;
    let arg4 = registers.r10;
    let arg5 = registers.r8;
    let arg6 = registers.r9;

    let res = match num {
        1 => write(arg1, arg2, arg3),
        _ => {
            log::warn!("unknown syscall {num}");
            Err(0)
        }
    };

    log::info!("syscall {num} ({arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6}) => {res:?}");

    registers.rax = match res {
        Ok(val) if val >> 63 == 0 => val,
        Ok(_) => {
            log::warn!("syscall {num} returned invalid {res:?}");
            u64::MAX
        }
        Err(val) if val >> 63 == 0 => val | 0x8000_0000_0000_0000,
        Err(_) => {
            log::warn!("syscall {num} returned invalid {res:?}");
            u64::MAX
        }
    };
}

fn write(fd: u64, ptr: u64, len: u64) -> Result<u64, u64> {
    if fd != 1 {
        return Err(1);
    }

    let bytes = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let string = core::str::from_utf8(bytes).map_err(|_| 2u64)?;
    print!("{}", string);

    Ok(string.len() as u64)
}
