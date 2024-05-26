use alloc::collections::BTreeMap;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

static MANAGER: Mutex<Manager> = Mutex::new(Manager {
    processes: BTreeMap::new(),
    next_process_id: 1,
});

pub struct Manager {
    processes: BTreeMap<ProcessId, Process>,
    next_process_id: u64,
}

impl Manager {
    pub fn get_process(&self, process_id: ProcessId) -> Option<&Process> {
        self.processes.get(&process_id)
    }

    pub fn get_process_mut(&mut self, process_id: ProcessId) -> Option<&mut Process> {
        self.processes.get_mut(&process_id)
    }
}

#[derive(Debug)]
pub struct Process {
    threads: BTreeMap<ThreadId, Thread>,
    process_id: ProcessId,
    next_thread_id: u64,
}

impl Process {
    pub fn get_thread(&self, thread_id: ThreadId) -> Option<&Thread> {
        self.threads.get(&thread_id)
    }

    pub fn get_thread_mut(&mut self, thread_id: ThreadId) -> Option<&mut Thread> {
        self.threads.get_mut(&thread_id)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(u64);

#[derive(Debug)]
pub struct Thread {
    context: ThreadContext,
    process_id: ProcessId,
    thread_id: ThreadId,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadId(u64);

#[derive(Debug)]
#[repr(C)]
pub struct ThreadContext {
    pub registers: Registers,
    pub stack_frame: InterruptStackFrame,
}

#[derive(Debug)]
#[repr(C)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}
