use core::pin::Pin;

use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};
use spin::{Mutex, Once};
use x86_64::{
    registers::{
        control::{Cr3, Cr3Flags},
        rflags::RFlags,
    },
    structures::{
        idt::InterruptStackFrame,
        paging::{Page, PageTable, PageTableFlags, PhysFrame},
    },
    VirtAddr,
};

use crate::{
    dbg,
    gdt::GDT,
    gsdata::KernelData,
    memory::{map_page, phys_to_virt, virt_to_phys},
};

pub static MANAGER: Once<Mutex<Manager>> = Once::new();

pub struct Manager {
    kernel_process: Arc<Mutex<Process>>,
    processes: BTreeMap<ProcessId, Arc<Mutex<Process>>>,
    next_process_id: u64,
    queue: VecDeque<Arc<Mutex<Thread>>>,
}

#[derive(Debug)]
pub struct Process {
    threads: BTreeMap<ThreadId, Arc<Mutex<Thread>>>,
    process_id: ProcessId,
    next_thread_id: u64,
    cr3: (PhysFrame, Cr3Flags),
    l4_table: Arc<Mutex<Pin<Box<PageTable>>>>,
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

        let kernel_process = Arc::new(Mutex::new(Process {
            threads: BTreeMap::new(),
            process_id: ProcessId(0),
            next_thread_id: 0,
            cr3,
            l4_table: Arc::new(Mutex::new(l4_table)),
        }));

        processes.insert(ProcessId(0), kernel_process.clone());

        Self {
            kernel_process,
            processes,
            next_process_id: 1,
            queue: VecDeque::new(),
        }
    }

    /// Call this function once on eash cpu
    /// It sets up cr3 and adds a thread to the kernel process.
    pub fn join_kernel(&mut self) -> Arc<Mutex<Thread>> {
        let mut kernel_process = self.kernel_process.lock();
        let cr3 = kernel_process.cr3;
        unsafe { Cr3::write(cr3.0, cr3.1) };
        kernel_process.join_kernel()
    }

    pub fn swap_thread(&mut self, active_context: &mut Context) {
        if let Some(new_thread) = self.queue.pop_front() {
            let core = KernelData::load_gsbase().unwrap();

            let old_thread = core.active_thread.clone();
            self.queue.push_back(old_thread.clone());
            old_thread.lock().context.clone_from(active_context);

            core.active_thread = new_thread.clone();
            let new_thread = new_thread.lock();
            active_context.clone_from(&new_thread.context);

            let process = self.get_process(new_thread.thread_id.0).unwrap();
            let process = process.lock();
            unsafe { Cr3::write(process.cr3.0, process.cr3.1) };
        }
    }

    pub fn kill_thread(&mut self, active_context: &mut Context) {
        let core = KernelData::load_gsbase().unwrap();

        let old_thread = core.active_thread.clone();
        let old_thread = old_thread.lock();

        log::info!("Killing thread {:?}", old_thread.thread_id);

        let old_process = self.get_process(old_thread.thread_id.0).unwrap();
        let mut old_process = old_process.lock();

        old_process.threads.remove(&old_thread.thread_id).unwrap();

        let new_thread = self.queue.pop_front().unwrap();
        core.active_thread = new_thread.clone();
        let new_thread = new_thread.lock();
        active_context.clone_from(&new_thread.context);

        let new_process = self.get_process(new_thread.thread_id.0).unwrap();
        let new_process = new_process.lock();
        unsafe { Cr3::write(new_process.cr3.0, new_process.cr3.1) };
    }

    pub fn get_process(&self, process_id: ProcessId) -> Option<Arc<Mutex<Process>>> {
        self.processes.get(&process_id).cloned()
    }

    pub fn get_thread(&self, id: ThreadId) -> Option<Arc<Mutex<Thread>>> {
        self.get_process(id.0)?.lock().get_thread(id)
    }

    pub fn get_kernel_l4_table(&self) -> Arc<Mutex<Pin<Box<PageTable>>>> {
        let process = self.kernel_process.clone();
        let process = process.lock();
        process.l4_table.clone()
    }

    pub fn spawn(&mut self, code: &[u8]) -> ThreadId {
        // Create new page table
        let kernel_l4_table = self.get_kernel_l4_table();
        let kernel_l4_table = kernel_l4_table.lock();

        let mut l4_table: Pin<Box<PageTable>> =
            Box::pin(kernel_l4_table.as_ref().get_ref().clone());
        let cr3 = {
            let l4_table_virt = VirtAddr::from_ptr(&*l4_table as *const _);
            let l4_table_phys = virt_to_phys(l4_table_virt).unwrap();
            (
                PhysFrame::containing_address(l4_table_phys),
                Cr3Flags::empty(),
            )
        };

        let page = Page::containing_address(VirtAddr::new(0x1000));
        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        let code_frame = map_page(page, flags, &mut l4_table);
        for i in 1..8 {
            map_page(page + i as u64, flags, &mut l4_table);
        }

        // todo: make this robust to non adjecent frames

        let code_dest: &mut [u8; 4096 * 8] =
            unsafe { &mut *phys_to_virt(code_frame.start_address()).as_mut_ptr() };

        code_dest[..code.len()].copy_from_slice(code);

        let process_id = ProcessId(self.next_process_id);
        self.next_process_id += 1;

        let thread_id = ThreadId(process_id, 0);

        let thread = Arc::new(Mutex::new(Thread {
            context: Context {
                registers: Registers::ZERO,
                stack_frame: InterruptStackFrame::new(
                    VirtAddr::new(0x1000),
                    GDT.user_code,
                    RFlags::INTERRUPT_FLAG,
                    VirtAddr::new(0x8000), // todo: get better stack location
                    GDT.user_data,
                ),
            },
            thread_id,
        }));

        let mut threads = BTreeMap::new();
        threads.insert(ThreadId(process_id, 0), thread.clone());

        let process = Process {
            threads,
            process_id,
            next_thread_id: 1,
            cr3,
            l4_table: Arc::new(Mutex::new(l4_table)),
        };

        self.processes
            .insert(process_id, Arc::new(Mutex::new(process)));
        self.queue.push_back(thread);

        thread_id
    }
}

impl Process {
    fn join_kernel(&mut self) -> Arc<Mutex<Thread>> {
        let thread_id = ThreadId(self.process_id, self.next_thread_id);
        self.next_thread_id += 1;

        let thread = Arc::new(Mutex::new(Thread {
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
        }));

        self.threads.insert(thread_id, thread.clone());

        thread
    }

    pub fn get_thread(&self, thread_id: ThreadId) -> Option<Arc<Mutex<Thread>>> {
        self.threads.get(&thread_id).cloned()
    }
}

impl Thread {
    pub fn id(&self) -> ThreadId {
        self.thread_id
    }
}

impl ProcessId {
    const KERNEL: Self = Self(0);
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
