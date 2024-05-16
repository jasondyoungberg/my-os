mod exception;
mod gdt;
mod hardware;
mod idt;
mod tss;

use idt::IDT;
use tss::TSS;
use x86_64::instructions::interrupts::{self};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    interrupts::disable();
    IDT.load();
    gdt::load();
    hardware::load();
    interrupts::enable();
}
