pub mod manager;
pub mod state;


use state::ProcessState;
use x86_64::{
    structures::paging::{PhysFrame},
};

#[derive(Debug)]
pub struct Process {
    pub state: ProcessState,
    pub cr3: PhysFrame,
}
