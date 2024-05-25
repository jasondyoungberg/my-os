use x86_64::structures::idt::InterruptStackFrame;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct ProcessState {
    pub registers: GeneralPurposeRegisters,
    pub stack_frame: InterruptStackFrame,
    // todo: save ymm and x87 registers
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C, align(16))]
pub struct GeneralPurposeRegisters {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    _rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}

impl Clone for ProcessState {
    fn clone(&self) -> Self {
        Self {
            registers: self.registers,
            stack_frame: InterruptStackFrame::new(
                self.stack_frame.instruction_pointer,
                self.stack_frame.code_segment,
                self.stack_frame.cpu_flags,
                self.stack_frame.stack_pointer,
                self.stack_frame.stack_segment,
            ),
        }
    }
}
