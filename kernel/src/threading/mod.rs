pub mod manager;
pub mod state;

use alloc::boxed::Box;
use core::pin::Pin;

use state::ProcessState;
use x86_64::{
    registers::control::Cr3Flags,
    structures::paging::{PageTable, PhysFrame},
};

#[derive(Debug)]
pub struct Process {
    pub state: ProcessState,
    pub cr3: PhysFrame,
}
