mod gdt;
mod idt;
mod tss;

use idt::IDT;
use tss::TSS;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    IDT.load();
    gdt::load();
}
