use core::fmt;

use x86_64::VirtAddr;

use crate::{process::Context, wrap};

wrap!(irq, debug_handler_inner => debug_handler);
pub extern "C" fn debug_handler_inner(context: &mut Context) {
    log::debug!(
        "Debug
rip: {} rsp: {} cs: {}  ss: {}
{:?}
rax: {} rbx: {} rcx: {} rdx: {}
rsi: {} rdi: {} rbp: {} rsp: {}
r8:  {} r9:  {} r10: {} r11: {}
r12: {} r13: {} r14: {} r15: {}
mm0: {} mm1: {} mm2: {} mm3: {}
mm4: {} mm5: {} mm6: {} mm7: {}
{}",
        HexFormat(context.stack_frame.instruction_pointer.as_u64()),
        HexFormat(context.stack_frame.stack_pointer.as_u64()),
        context.stack_frame.code_segment.index(),
        context.stack_frame.stack_segment.index(),
        context.stack_frame.cpu_flags,
        HexFormat(context.registers.rax),
        HexFormat(context.registers.rbx),
        HexFormat(context.registers.rcx),
        HexFormat(context.registers.rdx),
        HexFormat(context.registers.rsi),
        HexFormat(context.registers.rdi),
        HexFormat(context.registers.rbp),
        HexFormat(context.stack_frame.stack_pointer.as_u64()),
        HexFormat(context.registers.r8),
        HexFormat(context.registers.r9),
        HexFormat(context.registers.r10),
        HexFormat(context.registers.r11),
        HexFormat(context.registers.r12),
        HexFormat(context.registers.r13),
        HexFormat(context.registers.r14),
        HexFormat(context.registers.r15),
        HexFormat(context.registers.mm0),
        HexFormat(context.registers.mm1),
        HexFormat(context.registers.mm2),
        HexFormat(context.registers.mm3),
        HexFormat(context.registers.mm4),
        HexFormat(context.registers.mm5),
        HexFormat(context.registers.mm6),
        HexFormat(context.registers.mm7),
        Stack {
            rsp: context.stack_frame.stack_pointer,
            rbp: VirtAddr::new(context.registers.rbp),
            base: VirtAddr::new(0x8000),
        }
    );
}

pub struct HexFormat(pub u64);

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
