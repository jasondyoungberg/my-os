use spin::Lazy;
use x86_64::{
    registers::segmentation::GS,
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{
    exception::{double_fault_handler, page_fault_handler},
    hardware::{
        lapic,
        pics::{pics_handler, PICS_OFFSET},
    },
};

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
        unsafe { GS::swap() };
        todo!("handle irq {index} ({error_code:?})\n{stack_frame:#?}");
    }

    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    set_general_handler!(&mut idt, pics_handler, PICS_OFFSET..=PICS_OFFSET + 16);

    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(0);
        idt.page_fault
            .set_handler_fn(page_fault_handler)
            .set_stack_index(1);

        idt[lapic::TIMER_VECTOR]
            .set_handler_fn(lapic::handle_timer)
            .set_stack_index(2);
    };

    idt
});

#[macro_export]
macro_rules! wrap {
    (push) => {"
        push r15; push r14; push r13; push r12;
        push r11; push r10; push r9;  push r8;
                  push rbp; push rdi; push rsi;
        push rdx; push rcx; push rbx; push rax;
    "};
    (pop) => {"
        pop rax; pop rbx; pop rcx; pop rdx;
        pop rsi; pop rdi; pop rbp;
        pop r8;  pop r9;  pop r10; pop r11;
        pop r12; pop r13; pop r14; pop r15;
    "};
    (irq, $i:ident => $w: ident) => {
        const _: unsafe extern "C" fn(&mut $crate::process::Context) = $i;

        #[naked]
        pub extern "x86-interrupt" fn $w(
            _stack_frame: x86_64::structures::idt::InterruptStackFrame
        ) {
            unsafe {
                core::arch::asm!(
                    "swapgs",
                    $crate::wrap!(push),
                    "mov rdi, rsp",
                    "call {inner}",
                    $crate::wrap!(pop),
                    "swapgs",
                    "iretq",

                    inner = sym $i,
                    options(noreturn)
                )
            }
        }
    };
    (syscall, $i:ident => $w: ident) => {
        const _: unsafe extern "C" fn(&mut $crate::process::Registers) = $i;

        #[naked]
        pub extern "x86-interrupt" fn $w(
            _stack_frame: x86_64::structures::idt::InterruptStackFrame
        ) {
            unsafe {
                core::arch::asm!(
                    "swapgs",
                    "mov gs:8, rsp",
                    "mov rsp, gs:16",
                    $crate::wrap!(push),
                    "mov rdi, rsp",
                    "call {inner}",
                    $crate::wrap!(pop),
                    "mov rsp, gs:8",
                    "swapgs",
                    "sysretq",

                    inner = sym $i,
                    options(noreturn)
                )
            }
        }
    };
}
