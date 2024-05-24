mod breakpoint;
mod exception;
mod gdt;
mod hardware;
mod idt;
mod syscall;
mod tss;

pub use gdt::GDT_INFO;
use idt::IDT;
use tss::TSS;
use x86_64::instructions::interrupts::{self};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const BREAKPOINT_IST_INDEX: u16 = 1;
pub const PAGEFAULT_IST_INDEX: u16 = 2;

pub fn init() {
    interrupts::disable();
    IDT.load();
    gdt::load();
    hardware::load();
    interrupts::enable();
}
