mod breakpoint;
mod exception;
mod gdt;
mod hardware;
mod idt;
mod timer;
mod tss;

pub use gdt::GDT_INFO;
use idt::IDT;
use tss::TSS;
use x86_64::instructions::interrupts::{self};

pub fn init() {
    interrupts::disable();
    IDT.load();
    gdt::load();
    hardware::load();
    interrupts::enable();
}
