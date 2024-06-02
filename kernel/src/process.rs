use core::arch::asm;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use async_channel::{Receiver, Sender};
use crossbeam::queue::SegQueue;
use x86_64::{
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
    gdt::GDT,
    mapping::{map_page, new_page_table, MEMORY_OFFSET},
};

static QUEUE: SegQueue<Process> = SegQueue::new();

#[derive(Debug)]
pub struct Process {
    name: String,
    state: ProcessState,

    signal_down: Receiver<SignalDown>,
    signal_up: Sender<SignalUp>,

    children: Vec<Subprocess>,

    page_table_frame: PhysFrame,
    page_table: &'static mut PageTable,
    stack: PageRange,
}

impl Process {
    pub fn create_root(func: extern "C" fn() -> !) {
        let stack_start_addr = VirtAddr::new(0x1_0000_0000);
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
            signal_down: async_channel::unbounded::<SignalDown>().1,
            signal_up: async_channel::unbounded::<SignalUp>().0,
            children: Vec::new(),
            page_table,
            page_table_frame,
            stack,
        };

        QUEUE.push(root);
    }

    pub fn switch(
        active: &mut Option<Self>,
        stack_frame: &mut InterruptStackFrameValue,
        registers: &mut Registers,
    ) {
        if let Some(mut old) = active.take() {
            assert!(old.state.is_running(), "old process already paused");
            old.state = ProcessState::Paused {
                regs: registers.clone(),
                stack_frame: *stack_frame,
            };
            QUEUE.push(old);
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
                ProcessState::Running => panic!("new process already running"),
            }
            new.state = ProcessState::Running;

            unsafe { Cr3::write(new.page_table_frame, Cr3Flags::empty()) }

            active.replace(new);
        } else {
            stack_frame.instruction_pointer = VirtAddr::from_ptr(do_nothing as *const ());
        }
    }
}

#[derive(Debug)]
pub enum ProcessState {
    Running,
    Paused {
        regs: Registers,
        stack_frame: InterruptStackFrameValue,
    },
}
impl ProcessState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
}

#[derive(Debug)]
pub enum Subprocess {
    Active {
        signal_up: Receiver<SignalUp>,
        signal_down: Sender<SignalDown>,
    },
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
#[derive(Clone, Debug)]
pub struct Registers {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
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

#[naked]
extern "C" fn do_nothing() -> ! {
    unsafe {
        asm!(
            "
            2:
            hlt
            jmp 2b
            ",
            options(noreturn)
        );
    }
}
