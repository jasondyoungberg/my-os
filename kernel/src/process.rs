use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use async_channel::{Receiver, Sender};
use crossbeam::queue::SegQueue;
use spin::Lazy;
use x86_64::{
    instructions::hlt,
    registers::{
        control::{Cr3, Cr3Flags},
        rflags::RFlags,
    },
    structures::{
        idt::InterruptStackFrameValue,
        paging::{
            page::PageRange, FrameAllocator, OffsetPageTable, Page, PageTable, PageTableFlags,
            PhysFrame,
        },
    },
    VirtAddr,
};

use crate::{
    allocation::frame::MyFrameAllocator,
    gdt::{create_ministack, GDT},
    gsdata::GsData,
    mapping::{map_page, new_page_table, physical_to_virtual, MEMORY_OFFSET},
    SMP_RESPONSE,
};

static QUEUE: SegQueue<Process> = SegQueue::new();

#[derive(Debug)]
pub struct Process {
    name: String,
    state: ProcessState,

    signal_up: Sender<SignalUp>,
    signal_down: Receiver<SignalDown>,

    pub children: Vec<Subprocess>,

    page_table_frame: PhysFrame,
    page_table: &'static mut PageTable,
    stack: PageRange,
}

impl Process {
    pub fn create_root(func: extern "C" fn() -> !) {
        let stack_start_addr = VirtAddr::new(0xffff_a000_0000_0000);
        let stack_end_addr = stack_start_addr + 64 * 1024;
        let stack_page_start = Page::containing_address(stack_start_addr);
        let stack_page_end = Page::containing_address(stack_end_addr);
        let stack = Page::range(stack_page_start, stack_page_end);

        let page_table_frame = MyFrameAllocator
            .allocate_frame()
            .expect("no frames available");
        let page_table = unsafe { new_page_table(page_table_frame) };
        let mut mapper = unsafe { OffsetPageTable::new(page_table, VirtAddr::new(*MEMORY_OFFSET)) };
        for page in stack {
            unsafe {
                map_page(
                    &mut mapper,
                    page,
                    PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                );
            }
        }

        let root = Self {
            name: "root".to_string(),
            state: ProcessState::Paused {
                regs: Registers::ZERO,
                stack_frame: InterruptStackFrameValue::new(
                    VirtAddr::from_ptr(func as *const ()),
                    GDT.kernel_code,
                    RFlags::INTERRUPT_FLAG,
                    stack.end.start_address(),
                    GDT.kernel_data,
                ),
            },
            signal_up: async_channel::unbounded::<SignalUp>().0,
            signal_down: async_channel::unbounded::<SignalDown>().1,
            children: Vec::new(),
            page_table,
            page_table_frame,
            stack,
        };

        QUEUE.push(root);
    }

    pub fn create_user(&mut self, name: &str, code: &[u8]) -> usize {
        let stack_start_addr = VirtAddr::new(0x1_0000_0000);
        let stack_end_addr = stack_start_addr + 64 * 1024;
        let stack_page_start = Page::containing_address(stack_start_addr);
        let stack_page_end = Page::containing_address(stack_end_addr);
        let stack_pages = Page::range(stack_page_start, stack_page_end);

        let code_start_addr = VirtAddr::new(0x1_0000);
        let code_end_addr = code_start_addr + 64 * 1024;
        let code_page_start = Page::containing_address(code_start_addr);
        let code_page_end = Page::containing_address(code_end_addr);
        let code_pages = Page::range(code_page_start, code_page_end);

        let page_table_frame = MyFrameAllocator
            .allocate_frame()
            .expect("no frames available");
        let page_table = unsafe { new_page_table(page_table_frame) };
        let mut mapper = unsafe { OffsetPageTable::new(page_table, VirtAddr::new(*MEMORY_OFFSET)) };
        for page in stack_pages {
            unsafe {
                map_page(
                    &mut mapper,
                    page,
                    PageTableFlags::PRESENT
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::USER_ACCESSIBLE,
                );
            }
        }
        for (i, page) in code_pages.enumerate() {
            let frame = unsafe {
                map_page(
                    &mut mapper,
                    page,
                    PageTableFlags::PRESENT
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::USER_ACCESSIBLE,
                )
            };
            if i * 4096 < code.len() {
                let virt = physical_to_virtual(frame.start_address());
                let dest_ptr = virt.as_mut_ptr::<u8>();
                let src_ptr = code.as_ptr().wrapping_add(i * 4096);
                let len = core::cmp::min(4096, code.len() - i * 4096);
                unsafe { dest_ptr.copy_from_nonoverlapping(src_ptr, len) };
            }
        }

        let signal_down = async_channel::unbounded::<SignalDown>();
        let signal_up = async_channel::unbounded::<SignalUp>();

        let child_id = self.children.len();

        self.children.push(Subprocess {
            state: SubprocessState::Alive,
            name: name.to_string(),
            signal_up: signal_up.1,
            signal_down: signal_down.0,
        });

        let process = Self {
            name: name.to_string(),
            state: ProcessState::Paused {
                regs: Registers::ZERO,
                stack_frame: InterruptStackFrameValue::new(
                    VirtAddr::new(0x1_0000),
                    GDT.user_code,
                    RFlags::INTERRUPT_FLAG,
                    stack_pages.end.start_address(),
                    GDT.user_data,
                ),
            },
            signal_up: signal_up.0,
            signal_down: signal_down.1,
            children: Vec::new(),
            page_table,
            page_table_frame,
            stack: stack_pages,
        };

        QUEUE.push(process);

        child_id
    }

    pub fn exit(&mut self, code: i64) {
        self.state = ProcessState::Dying;
        let _ = self.signal_up.try_send(SignalUp::Exit(code));
    }

    fn update(&mut self) {
        while let Ok(signal) = self.signal_down.try_recv() {
            match signal {
                SignalDown::Kill => {
                    self.state = ProcessState::Dying;
                    let _ = self.signal_up.try_send(SignalUp::Exit(-1));
                }
            }
        }

        for child in self.children.iter_mut() {
            while let Ok(signal) = child.signal_up.try_recv() {
                match signal {
                    SignalUp::Exit(code) => {
                        child.state = SubprocessState::Dead(code);
                    }
                }
            }
        }
    }

    pub fn switch(stack_frame: &mut InterruptStackFrameValue, registers: &mut Registers) {
        let gsdata = GsData::load().unwrap();
        let mut active_process = gsdata.process.lock();

        if let Some(mut old) = active_process.take() {
            match old.state {
                ProcessState::Running => {
                    old.state = ProcessState::Paused {
                        regs: registers.clone(),
                        stack_frame: *stack_frame,
                    };
                    QUEUE.push(old);
                }
                ProcessState::Dying => (),
                ProcessState::Paused {
                    regs: _,
                    stack_frame: _,
                } => {
                    log::error!("old process already paused: {:?}", old.name);
                    old.state = ProcessState::Paused {
                        regs: registers.clone(),
                        stack_frame: *stack_frame,
                    };
                    QUEUE.push(old);
                }
            }
        }

        if let Some(mut new) = QUEUE.pop() {
            match new.state {
                ProcessState::Paused {
                    regs,
                    stack_frame: new_stack_frame,
                } => {
                    *stack_frame = new_stack_frame;
                    *registers = regs;
                }
                ProcessState::Dying => panic!("new process already dying"),
                ProcessState::Running => panic!("new process already running"),
            }
            new.state = ProcessState::Running;

            unsafe { Cr3::write(new.page_table_frame, Cr3Flags::empty()) }

            new.update();

            let old_process = active_process.replace(new);
            if let Some(old_process) = old_process {
                log::error!("old process not taken: {:?}", old_process.name);
            }
        } else {
            let cpuid = gsdata.cpuid;
            *stack_frame = InterruptStackFrameValue::new(
                VirtAddr::from_ptr(do_nothing as *const ()),
                GDT.kernel_code,
                RFlags::INTERRUPT_FLAG,
                NOTHING_STACKS[cpuid],
                GDT.kernel_data,
            );
        }
    }
}

#[derive(Debug)]
pub enum ProcessState {
    Running,
    Dying,
    Paused {
        regs: Registers,
        stack_frame: InterruptStackFrameValue,
    },
}

#[derive(Debug)]
pub struct Subprocess {
    pub name: String,
    pub state: SubprocessState,
    signal_up: Receiver<SignalUp>,
    signal_down: Sender<SignalDown>,
}

#[derive(Debug)]
pub enum SubprocessState {
    Alive,
    Dead(i64),
}

#[derive(Debug)]
pub enum SignalDown {
    Kill,
}

#[derive(Debug)]
pub enum SignalUp {
    Exit(i64),
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
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
impl Registers {
    const ZERO: Registers = Registers {
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

fn do_nothing() -> ! {
    loop {
        hlt();
    }
}

static NOTHING_STACKS: Lazy<Vec<VirtAddr>> = Lazy::new(|| {
    let cpu_count = SMP_RESPONSE.cpus().len();
    let mut stacks = Vec::with_capacity(cpu_count);
    for _ in 0..cpu_count {
        stacks.push(create_ministack());
    }
    stacks
});
