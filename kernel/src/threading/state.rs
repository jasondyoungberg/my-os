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
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
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
