use core::arch::asm;

use spin::Lazy;
use x86_64::{
    registers::control::Cr2,
    set_general_handler,
    structures::idt::{
        InterruptDescriptorTable, InterruptStackFrame, InterruptStackFrameValue, PageFaultErrorCode,
    },
};

use crate::{
    drivers::lapic,
    gsdata::GsData,
    println,
    process::{Process, Registers},
};

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    fn my_general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
        if let Some(error_code) = error_code {
            todo!(
                "handle irq {} ({:?})\n{:#?}",
                index,
                error_code,
                stack_frame
            );
        } else {
            todo!("handle irq {}\n{:#?}", index, stack_frame);
        }
    }

    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, my_general_handler);

    unsafe {
        idt.breakpoint.set_handler_fn(breakpoint);
        idt.page_fault.set_handler_fn(page_fault).set_stack_index(1);
        idt.double_fault
            .set_handler_fn(double_fault)
            .set_stack_index(2);
        idt[lapic::TIMER_VECTOR].set_handler_fn(timer);
    }

    idt
});

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint(_stack_frame: InterruptStackFrame) {
    println!("Breakpoint");
}

extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!(
        "\
Page fault
Accessed Address: {:?}
Error code: {:?}
{:#?}",
        Cr2::read(),
        error_code,
        stack_frame
    );
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Double fault\n{:#?}", stack_frame);
}

#[naked]
extern "x86-interrupt" fn timer(_stack_frame: InterruptStackFrame) {
    unsafe {
        asm!("
            swapgs

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

            lea rdi, [rsp + 8*15]
            mov rsi, rsp
            call {timer_inner}

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

            swapgs
            iretq
            ",
            timer_inner = sym timer_inner,
            options(noreturn)
        );
    }
}

extern "C" fn timer_inner(stack_frame: &mut InterruptStackFrameValue, registers: &mut Registers) {
    Process::switch(stack_frame, registers);

    let gsdata = GsData::load().expect("Unable to load gsdata");
    gsdata.lapic.lock().signal_eoi();
}
