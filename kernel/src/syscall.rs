use core::arch::asm;

use alloc::string::String;
use x86_64::{
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask, Star},
        rflags::RFlags,
    },
    structures::idt::InterruptStackFrameValue,
    VirtAddr,
};

use crate::{
    gdt::GDT,
    gsdata::GsData,
    print,
    process::{Process, Registers},
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
    SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG);

    unsafe { Efer::write(Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS) };
}

#[naked]
extern "C" fn handle_syscall(registers: &mut Registers) {
    unsafe {
        asm!(
            "
            swapgs
            mov gs:24, rsp
            mov rsp, gs:16

            push r15
            push r14
            push r13
            push r12
            push r11
            push r10
            push r9
            push r8
            push rbp
            push rdi
            push rsi
            push rdx
            push rcx
            push rbx
            push rax

            mov rdi, rsp
            call {inner}

            pop rax
            pop rbx
            pop rcx
            pop rdx
            pop rsi
            pop rdi
            pop rbp
            pop r8
            pop r9
            pop r10
            pop r11
            pop r12
            pop r13
            pop r14
            pop r15

            mov rsp, gs:24
            swapgs
            sysretq
            ",
            inner = sym handle_syscall_inner,
            options(noreturn)
        );
    }
}

extern "C" fn handle_syscall_inner(registers: &mut Registers) {
    let num = registers.rax;
    let arg1 = registers.rdi;
    let arg2 = registers.rsi;
    let arg3 = registers.rdx;
    let arg4 = registers.r10;
    let arg5 = registers.r8;
    let arg6 = registers.r9;

    log::trace!("syscall {num} ({arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6})");

    let ret = match num {
        1 => write(arg1, arg2, arg3),
        24 => sched_yield(registers),
        _ => {
            log::warn!("unknown syscall {num}");
            0
        }
    };

    log::trace!("sysret {ret}");

    registers.rax = ret;
}

fn write(_fd: u64, buf: u64, len: u64) -> u64 {
    let buf = buf as *const u8;
    let len = len as usize;

    let slice = unsafe { core::slice::from_raw_parts(buf, len) };
    let string = String::from_utf8_lossy(slice);

    print!("{}", string);

    0
}

fn sched_yield(registers: &mut Registers) -> u64 {
    log::trace!("sched_yield");
    let ret = 0;

    registers.rax = ret;

    let gsdata = unsafe { GsData::load().unwrap() };
    let sysret_rsp = gsdata.sysret_rsp;

    let mut stack_frame = InterruptStackFrameValue::new(
        VirtAddr::new(registers.rcx),
        GDT.user_code,
        RFlags::from_bits_retain(registers.r11),
        sysret_rsp,
        GDT.user_data,
    );

    fake_irq(&mut stack_frame, registers);
}

#[naked]
extern "C" fn fake_irq(stack_frame: &mut InterruptStackFrameValue, registers: &mut Registers) -> ! {
    unsafe {
        asm!(
            "
            push [rdi + 4*8]
            push [rdi + 3*8]
            push [rdi + 2*8]
            push [rdi + 1*8]
            push [rdi + 0*8]

            push [rsi + 14*8]
            push [rsi + 13*8]
            push [rsi + 12*8]
            push [rsi + 11*8]
            push [rsi + 10*8]
            push [rsi + 9*8]
            push [rsi + 8*8]
            push [rsi + 6*8]
            push [rsi + 7*8]
            push [rsi + 5*8]
            push [rsi + 4*8]
            push [rsi + 3*8]
            push [rsi + 2*8]
            push [rsi + 1*8]
            push [rsi + 0*8]

            lea rdi, [rsp + 8*15]
            mov rsi, rsp
            call {inner}
            swapgs

            pop rax
            pop rbx
            pop rcx
            pop rdx
            pop rsi
            pop rdi
            pop rbp
            pop r8
            pop r9
            pop r10
            pop r11
            pop r12
            pop r13
            pop r14
            pop r15

            iretq
            ",
            inner = sym fake_irq_inner,
            options(noreturn)
        )
    }
}

extern "C" fn fake_irq_inner(
    stack_frame: &mut InterruptStackFrameValue,
    registers: &mut Registers,
) {
    let gsdata = unsafe { GsData::load().unwrap() };
    Process::switch(&mut gsdata.process, stack_frame, registers);
}
