use alloc::collections::BTreeMap;
use spin::{Lazy, Mutex};
use x86_64::{
    registers::{
        control::{Cr3, Cr3Flags},
        rflags::RFlags,
    },
    structures::{
        gdt::SegmentSelector,
        idt::InterruptStackFrame,
        paging::{
            FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        },
    },
    PrivilegeLevel::Ring0,
    VirtAddr,
};

use crate::memory::phys_to_ptr;

use super::{
    state::{GeneralPurposeRegisters, ProcessState},
    Process,
};

pub static MANAGER: Lazy<Mutex<ProcessManager>> = Lazy::new(|| {
    let mut processes = BTreeMap::new();
    let active_pid = ProcessId::new();

    let (cr3, _) = Cr3::read();

    let kernel_process = Process {
        state: ProcessState {
            registers: GeneralPurposeRegisters::default(),
            stack_frame: InterruptStackFrame::new(
                VirtAddr::new(0),
                SegmentSelector::new(0, Ring0),
                RFlags::empty(),
                VirtAddr::new(0),
                SegmentSelector::new(0, Ring0),
            ),
        },
        cr3,
    };

    processes.insert(active_pid, kernel_process);

    Mutex::new(ProcessManager {
        processes,
        active_pid,
        kernel_cr3: cr3,
    })
});

pub struct ProcessManager {
    processes: BTreeMap<ProcessId, Process>,
    active_pid: ProcessId,
    kernel_cr3: PhysFrame,
}

impl ProcessManager {
    pub fn spawn(&mut self, code: &[u8]) -> ProcessId {
        let id = ProcessId::new();

        let mut frame_allocator = crate::memory::FRAME_ALLOCATOR.get().unwrap().lock();

        let page_table_flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        let cr3 = frame_allocator
            .allocate_frame()
            .expect("no frames available");

        let l4_table: &mut PageTable = unsafe { phys_to_ptr(cr3.start_address()) };

        l4_table.clone_from(unsafe { phys_to_ptr(self.kernel_cr3.start_address()) });

        let mut mapper = unsafe {
            OffsetPageTable::new(
                l4_table,
                VirtAddr::new(crate::memory::PHYSICAL_MEMORY_OFFSET),
            )
        };

        let code_frame = frame_allocator
            .allocate_frame()
            .expect("no frames available");
        unsafe {
            mapper.map_to(
                Page::containing_address(VirtAddr::new(0x1000)),
                code_frame,
                page_table_flags,
                &mut *frame_allocator,
            )
        }
        .unwrap()
        .flush();

        let code_ptr: &mut [u8; 4096] = unsafe { phys_to_ptr(code_frame.start_address()) };

        code_ptr[..code.len()].copy_from_slice(code);

        for i in 3..16 {
            let frame = frame_allocator
                .allocate_frame()
                .expect("no frames available");

            unsafe {
                mapper.map_to(
                    Page::containing_address(VirtAddr::new(i * 4096)),
                    frame,
                    page_table_flags,
                    &mut *frame_allocator,
                )
            }
            .unwrap()
            .flush();
        }

        let process = Process {
            state: ProcessState {
                registers: GeneralPurposeRegisters::default(),
                stack_frame: InterruptStackFrame::new(
                    VirtAddr::new(0x1000),
                    crate::interrupts::GDT_INFO.user_code_selector,
                    RFlags::INTERRUPT_FLAG,
                    // RFlags::empty(),
                    VirtAddr::new(0x1_0000),
                    crate::interrupts::GDT_INFO.user_data_selector,
                ),
            },
            cr3,
        };

        self.processes.insert(id, process);
        id
    }

    pub fn next(&mut self, active_state: &mut ProcessState) {
        let old_pid = self.active_pid;
        let new_pid = self
            .processes
            .range(&self.active_pid..)
            .nth(1)
            .map(|(pid, _)| *pid)
            .or_else(|| self.processes.keys().next().copied())
            .unwrap_or(self.active_pid);

        let old_process = self.processes.get_mut(&old_pid).unwrap();
        old_process.state = active_state.clone();

        let new_process = self.processes.get_mut(&new_pid).unwrap();

        self.active_pid = new_pid;
        active_state.clone_from(&new_process.state);

        unsafe {
            Cr3::write(new_process.cr3, Cr3Flags::empty());
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ProcessId(u64);

impl ProcessId {
    pub fn new() -> Self {
        use core::sync::atomic::{AtomicU64, Ordering};

        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
