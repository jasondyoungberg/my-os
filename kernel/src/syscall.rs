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
    color::Color,
    console,
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
    SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG);

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

    log::info!("syscall {num} ({arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6})");

    let res = match num {
        1 => write(arg1, arg2, arg3),
        24 => sys_yield(registers),
        60 => exit(arg1, registers),
        _ => {
            log::warn!("unknown syscall {num}");
            Err(0)
        }
    };

    log::info!("sysret {res:?}");

    registers.rax = result_to_u64(res);
}

fn result_to_u64(res: Result<u64, u64>) -> u64 {
    match res {
        Ok(val) if val >> 63 == 0 => val,
        Ok(_) => 0,
        Err(val) if val >> 63 == 0 => val | 0x8000_0000_0000_0000,
        Err(_) => 0,
    }
}

fn write(fd: u64, ptr: u64, len: u64) -> Result<u64, u64> {
    let bytes = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let string = core::str::from_utf8(bytes).map_err(|_| 2u64)?;
    match fd {
        1 => {
            print!("{}", string);
            Ok(string.len() as u64)
        }
        2 => {
            let mut console = console::CONSOLE.lock();
            console.set_colors(Color::RED, Color::BLACK);
            console.write_str(string);
            console.set_colors(Color::WHITE, Color::BLACK);
            console.flush();
            Ok(string.len() as u64)
        }
        _ => Err(1),
    }
}

fn sys_yield(registers: &mut Registers) -> Result<u64, u64> {
    let res = Ok(0);

    let mut fake_context = build_fake_context(res, registers);

    fake_irq(&mut fake_context, FakeIrqAction::SwapThread);
}

fn exit(_code: u64, registers: &mut Registers) -> Result<u64, u64> {
    let res = Ok(0);

    let mut fake_context = build_fake_context(res, registers);

    fake_irq(&mut fake_context, FakeIrqAction::KillThread);
}

#[derive(Debug)]
#[repr(u64)]
enum FakeIrqAction {
    SwapThread = 0,
    KillThread = 1,
}

fn build_fake_context(res: Result<u64, u64>, registers: &mut Registers) -> Context {
    let kernel_data = KernelData::load_gsbase().unwrap();

    registers.rax = result_to_u64(res);

    Context {
        registers: *registers,
        stack_frame: InterruptStackFrame::new(
            VirtAddr::new(registers.rcx),
            GDT.user_code,
            RFlags::from_bits_retain(registers.r11),
            kernel_data.sysret_stack,
            GDT.user_data,
        ),
    }
}

#[naked]
extern "C" fn fake_irq(context: &mut Context, action: FakeIrqAction) -> ! {
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

extern "C" fn fake_irq_inner(context: &mut Context, action: FakeIrqAction) {
    log::trace!("fake interrupt: {action:?}");

    let mut manager = MANAGER.get().unwrap().lock();
    match action {
        FakeIrqAction::SwapThread => manager.swap_thread(context),
        FakeIrqAction::KillThread => manager.kill_thread(context),
    };
}
