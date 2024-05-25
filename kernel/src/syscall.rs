use stdlib::syscall::SyscallId;
use x86_64::{
    registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask, Star},
        rflags::RFlags,
    },
    VirtAddr,
};

use crate::{kprint, memory::MINI_STACK_SIZE, threading::state::GeneralPurposeRegisters};

#[repr(align(4096), C)]
struct Stack([u8; MINI_STACK_SIZE]);

static mut SYSCALL_STACK: Stack = Stack([0; MINI_STACK_SIZE]);

pub fn init() {
    LStar::write(VirtAddr::new(syscall_handler as usize as u64));
    Star::write(
        crate::interrupts::GDT_INFO.user_code_selector,
        crate::interrupts::GDT_INFO.user_data_selector,
        crate::interrupts::GDT_INFO.kernel_code_selector,
        crate::interrupts::GDT_INFO.kernel_data_selector,
    )
    .unwrap();
    SFMask::write(RFlags::INTERRUPT_FLAG);

    let efer = Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS;

    unsafe { Efer::write(efer) };
}

#[naked]
extern "C" fn syscall_handler() {
    // code based on https://github.com/rust-osdev/x86_64/issues/392#issuecomment-1257883895
    unsafe {
        core::arch::asm! {"
                mov r15, rsp
                lea rsp, [rip + {stack} + {stack_size}]

                push r15
                push r14
                push r13
                push r12
                push r11
                push r10
                push r9
                push r8
                push 0 // skip rsp
                push rbp
                push rdi
                push rsi
                push rdx
                push rcx
                push rbx
                push rax

                // 1st arg (rdi): state
                mov rdi, rsp
                call {inner}

                pop rax
                pop rbx
                pop rcx
                pop rdx
                pop rsi
                pop rdi
                pop rbp
                add rsp, 8 // skip rsp
                pop r8
                pop r9
                pop r10
                pop r11
                pop r12
                pop r13
                pop r14
                pop r15

                mov rsp, r15

                sysretq
            ",

            stack = sym SYSCALL_STACK,
            stack_size = const MINI_STACK_SIZE,
            inner = sym syscall_handler_inner,

            options(noreturn)
        };
    }
}

extern "C" fn syscall_handler_inner(state: &mut GeneralPurposeRegisters) {
    let n = state.rax;
    let arg1 = state.rdi;
    let arg2 = state.rsi;
    let arg3 = state.rdx;
    let arg4 = state.r10;
    let arg5 = state.r8;
    let arg6 = state.r9;

    log::info!("syscall {n} ({arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6})");

    let result: Result<u64, u64> = match n.try_into() {
        Ok(SyscallId::Exit) => todo!(),
        Ok(SyscallId::Read) => todo!(),
        Ok(SyscallId::Write) => write(arg1, arg2),
        Err(_) => Err(0),
    };

    log::info!("sysret {result:?}");

    state.rax = match result {
        Ok(value) => value & 0x7fff_ffff_ffff_ffff,
        Err(error) => error | 0x8000_0000_0000_0000,
    };
}

fn write(buf: u64, count: u64) -> Result<u64, u64> {
    #[allow(clippy::cast_possible_truncation)]
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, count as usize) };
    let string = core::str::from_utf8(buf).map_err(|_| 1u64)?;
    kprint!("{}", string);
    Ok(count)
}
