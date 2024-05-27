use core::pin::Pin;

use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
};
use spin::{Mutex, Once};
use x86_64::{
    registers::{
        control::{Cr3, Cr3Flags},
        rflags::RFlags,
    },
    structures::{
        idt::InterruptStackFrame,
        paging::{
            FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        },
    },
    VirtAddr,
};

use crate::{
    dbg,
    gdt::GDT,
    gsdata::KernelData,
    memory::{phys_to_virt, virt_to_phys, MemoryMapFrameAllocator, MEMORY_OFFSET},
};

pub static MANAGER: Once<Mutex<Manager>> = Once::new();

pub struct Manager {
    processes: BTreeMap<ProcessId, Process>,
    next_process_id: u64,
    queue: VecDeque<ThreadId>,
}

#[derive(Debug)]
pub struct Process {
    threads: BTreeMap<ThreadId, Thread>,
    process_id: ProcessId,
    next_thread_id: u64,
    cr3: (PhysFrame, Cr3Flags),
    l4_table: Pin<Box<PageTable>>,
}

#[derive(Debug)]
pub struct Thread {
    context: Context,
    thread_id: ThreadId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadId(ProcessId, u64);

#[derive(Debug)]
#[repr(C)]
pub struct Context {
    pub registers: Registers,
    pub stack_frame: InterruptStackFrame,
}

#[derive(Clone, Copy, Debug)]
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

impl Manager {
    /// This function initializes the process manager.
    /// It should be called only once.
    pub fn init() -> Self {
        let mut processes = BTreeMap::new();

        let l4_table = {
            let (frame, _) = Cr3::read();
            let ptr: *mut PageTable = phys_to_virt(frame.start_address()).as_mut_ptr();
            let old_table = unsafe { &mut *ptr };
            Box::pin(old_table.clone())
        };

        let cr3 = {
            let l4_table_virt = VirtAddr::from_ptr(&*l4_table as *const _);
            let l4_table_phys = virt_to_phys(l4_table_virt).unwrap();
            dbg!(l4_table_virt, l4_table_phys);
            (
                PhysFrame::containing_address(l4_table_phys),
                Cr3Flags::empty(),
            )
        };

        log::info!("Cr3 {:?} => {:?}", Cr3::read(), cr3);

        processes.insert(
            ProcessId(0),
            Process {
                threads: BTreeMap::new(),
                process_id: ProcessId(0),
                next_thread_id: 0,
                cr3,
                l4_table,
            },
        );

        Self {
            processes,
            next_process_id: 1,
            queue: VecDeque::new(),
        }
    }

    pub fn join_kernel(&mut self) -> ThreadId {
        let kernel_process = self.kernel_process_mut();
        let cr3 = kernel_process.cr3;
        unsafe { Cr3::write(cr3.0, cr3.1) };
        kernel_process.join_kernel()
    }

    pub fn swap_task(&mut self, core: &mut KernelData, active_context: &mut Context) {
        if let Some(new_thread_id) = self.queue.pop_front() {
            let old_thread_id = core.active_thread;
            self.queue.push_back(old_thread_id);

            let old_thread = self.get_thread_mut(old_thread_id).unwrap();
            old_thread.context.clone_from(active_context);

            let new_thread = self.get_thread_mut(new_thread_id).unwrap();
            core.active_thread = new_thread_id;
            active_context.clone_from(&new_thread.context);

            let process = self.get_process_mut(new_thread_id.0).unwrap();
            unsafe { Cr3::write(process.cr3.0, process.cr3.1) };
        }
    }

    pub fn get_process(&self, process_id: ProcessId) -> Option<&Process> {
        self.processes.get(&process_id)
    }

    pub fn get_process_mut(&mut self, process_id: ProcessId) -> Option<&mut Process> {
        self.processes.get_mut(&process_id)
    }

    pub fn get_thread(&self, id: ThreadId) -> Option<&Thread> {
        self.get_process(id.0)?.get_thread(id)
    }

    pub fn get_thread_mut(&mut self, id: ThreadId) -> Option<&mut Thread> {
        self.get_process_mut(id.0)?.get_thread_mut(id)
    }

    pub fn kernel_process(&self) -> &Process {
        self.get_process(ProcessId(0)).unwrap()
    }

    pub fn kernel_process_mut(&mut self) -> &mut Process {
        self.get_process_mut(ProcessId(0)).unwrap()
    }

    pub fn kernel_l4_table(&self) -> &PageTable {
        &self.kernel_process().l4_table
    }

    pub fn kernel_l4_table_mut(&mut self) -> &mut PageTable {
        &mut self.kernel_process_mut().l4_table
    }

    pub fn spawn(&mut self, code: &[u8]) -> ThreadId {
        // Create new page table
        let mut l4_table: Pin<Box<PageTable>> = Box::pin(self.kernel_l4_table().clone());
        let cr3 = {
            let l4_table_virt = VirtAddr::from_ptr(&*l4_table as *const _);
            let l4_table_phys = virt_to_phys(l4_table_virt).unwrap();
            (
                PhysFrame::containing_address(l4_table_phys),
                Cr3Flags::empty(),
            )
        };

        let mut frame_allocator = MemoryMapFrameAllocator;
        let l4_table_ref = &mut *l4_table.as_mut();
        let mut mapper = unsafe { OffsetPageTable::new(l4_table_ref, *MEMORY_OFFSET) };

        // Write code to memory

        let code_frame = frame_allocator
            .allocate_frame()
            .expect("no frames available");

        unsafe {
            mapper.map_to(
                Page::containing_address(VirtAddr::new(0x1000)),
                code_frame,
                PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE,
                &mut frame_allocator,
            )
        }
        .unwrap()
        .flush();

        let code_dest: &mut [u8; 4096] =
            unsafe { &mut *phys_to_virt(code_frame.start_address()).as_mut_ptr() };

        code_dest[..code.len()].copy_from_slice(code);

        let process_id = ProcessId(self.next_process_id);
        self.next_process_id += 1;

        let thread_id = ThreadId(process_id, 0);

        let thread = Thread {
            context: Context {
                registers: Registers::ZERO,
                stack_frame: InterruptStackFrame::new(
                    VirtAddr::new(0x1000),
                    GDT.user_code,
                    RFlags::INTERRUPT_FLAG,
                    VirtAddr::zero(), // todo: set stack pointer
                    GDT.user_data,
                ),
            },
            thread_id,
        };

        let mut threads = BTreeMap::new();
        threads.insert(ThreadId(process_id, 0), thread);

        let process = Process {
            threads,
            process_id,
            next_thread_id: 1,
            cr3,
            l4_table,
        };

        self.processes.insert(process_id, process);
        self.queue.push_back(thread_id);

        let thread_id = self.get_process_mut(process_id).unwrap().join_kernel();

        thread_id
    }
}

impl Process {
    fn join_kernel(&mut self) -> ThreadId {
        let thread_id = ThreadId(self.process_id, self.next_thread_id);
        self.next_thread_id += 1;

        let thread = Thread {
            context: Context {
                registers: Registers::ZERO,
                stack_frame: InterruptStackFrame::new(
                    VirtAddr::zero(),
                    GDT.kernel_code,
                    RFlags::INTERRUPT_FLAG,
                    VirtAddr::zero(),
                    GDT.kernel_data,
                ),
            },
            thread_id,
        };

        self.threads.insert(thread.thread_id, thread);

        thread_id
    }

    pub fn get_thread(&self, thread_id: ThreadId) -> Option<&Thread> {
        self.threads.get(&thread_id)
    }

    pub fn get_thread_mut(&mut self, thread_id: ThreadId) -> Option<&mut Thread> {
        self.threads.get_mut(&thread_id)
    }
}

impl Clone for Context {
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

impl Registers {
    pub const ZERO: Self = Self {
        rax: 0,
        rbx: 0,
        rcx: 0,
        rdx: 0,
        rsi: 0,
        rdi: 0,
        rbp: 0,
        r8: 0,
        r9: 0,
        r10: 0,
        r11: 0,
        r12: 0,
        r13: 0,
        r14: 0,
        r15: 0,
    };
}