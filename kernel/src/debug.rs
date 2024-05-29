use core::fmt;

use x86_64::VirtAddr;

use crate::{process::Context, wrap};

wrap!(irq, debug_handler_inner => debug_handler);
pub extern "C" fn debug_handler_inner(context: &mut Context) {
    let rip = HexFormat(context.stack_frame.instruction_pointer.as_u64());
    let rsp = HexFormat(context.stack_frame.stack_pointer.as_u64());
    let cs = context.stack_frame.code_segment.index();
    let ss = context.stack_frame.stack_segment.index();

    let rflags = context.stack_frame.cpu_flags;
    let rax = HexFormat(context.registers.rax);
    let rbx = HexFormat(context.registers.rbx);
    let rcx = HexFormat(context.registers.rcx);
    let rdx = HexFormat(context.registers.rdx);
    let rsi = HexFormat(context.registers.rsi);
    let rdi = HexFormat(context.registers.rdi);
    let rbp = HexFormat(context.registers.rbp);
    let r8 = HexFormat(context.registers.r8);
    let r9 = HexFormat(context.registers.r9);
    let r10 = HexFormat(context.registers.r10);
    let r11 = HexFormat(context.registers.r11);
    let r12 = HexFormat(context.registers.r12);
    let r13 = HexFormat(context.registers.r13);
    let r14 = HexFormat(context.registers.r14);
    let r15 = HexFormat(context.registers.r15);

    let stack = Stack {
        rsp: context.stack_frame.stack_pointer,
        rbp: VirtAddr::new(context.registers.rbp),
        base: VirtAddr::new(0x8000),
    };

    log::debug!(
        "Debug
RIP: {rip} RSP: {rsp} CS: {cs}  SS: {ss}
{rflags:?}
RAX: {rax} RBX: {rbx} RCX: {rcx} RDX: {rdx}
RSI: {rsi} RDI: {rdi} RBP: {rbp} RSP: {rsp}
R8:  {r8} R9:  {r9} R10: {r10} R11: {r11}
R12: {r12} R13: {r13} R14: {r14} R15: {r15}
{stack}"
    );
}

struct HexFormat(u64);

impl fmt::Display for HexFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 0 {
            write!(f, "                   ")
        } else {
            write!(
                f,
                "{:04x}_{:04x}_{:04x}_{:04x}",
                (self.0 >> 48) & 0xFFFF,
                (self.0 >> 32) & 0xFFFF,
                (self.0 >> 16) & 0xFFFF,
                self.0 & 0xFFFF
            )
        }
    }
}

struct Stack {
    rsp: VirtAddr,
    rbp: VirtAddr,
    base: VirtAddr,
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start_addr = self.rsp.as_u64() - 8 * 8;
        let end_addr = self.base.as_u64();

        for addr in (start_addr..end_addr).step_by(8) {
            let addr = VirtAddr::new(addr);
            let value = unsafe { *addr.as_ptr() };
            write!(f, "{}: {}", HexFormat(addr.as_u64()), HexFormat(value))?;

            if addr == self.rsp && addr == self.rbp {
                write!(f, " <- RSP, RBP")?;
            } else if addr == self.rsp {
                write!(f, " <- RSP")?;
            } else if addr == self.rbp {
                write!(f, " <- RBP")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
