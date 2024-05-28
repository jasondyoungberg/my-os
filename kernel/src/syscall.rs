use core::arch::asm;

use x86_64::{
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask, Star},
        rflags::RFlags,
    },
    structures::idt::InterruptStackFrame,
    VirtAddr,
};

use crate::{
    gdt::GDT,
    gsdata::KernelData,
    print,
    process::{Context, Registers, MANAGER},
    wrap,
};

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

    log::debug!("syscall {num} ({arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6})");

    let res = match num {
        1 => write(arg1, arg2, arg3),
        4 => task_switch(sleep(arg1), registers),
        _ => {
            log::warn!("unknown syscall {num}");
            Err(0)
        }
    };

    log::debug!("sysret {res:?}");

    registers.rax = result_to_u64(res);
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

fn sleep(_ms: u64) -> Result<u64, u64> {
    Ok(0)
}

fn result_to_u64(res: Result<u64, u64>) -> u64 {
    match res {
        Ok(val) if val >> 63 == 0 => val,
        Ok(_) => 0,
        Err(val) if val >> 63 == 0 => val | 0x8000_0000_0000_0000,
        Err(_) => 0,
    }
}

fn task_switch(res: Result<u64, u64>, registers: &mut Registers) -> Result<u64, u64> {
    let kernel_data = KernelData::load_gsbase().unwrap();

    registers.rax = result_to_u64(res);

    let mut fake_context = Context {
        registers: *registers,
        stack_frame: InterruptStackFrame::new(
            VirtAddr::new(registers.rcx),
            GDT.user_code,
            RFlags::from_bits_retain(registers.r11),
            kernel_data.sysret_stack,
            GDT.user_data,
        ),
    };

    fake_irq(&mut fake_context);
}

#[naked]
extern "C" fn fake_irq(context: &mut Context) -> ! {
    unsafe {
        asm!(
            "mov rsp, rdi",
            "call {inner}",
            wrap!(pop),
            "swapgs",
            "iretq",
            inner = sym fake_irq_inner,
            options(noreturn)
        );
    }
}

extern "C" fn fake_irq_inner(context: &mut Context) {
    log::trace!("fake interrupt");
    MANAGER.get().unwrap().lock().swap_thread(context);
}
