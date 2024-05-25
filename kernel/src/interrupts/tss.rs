use spin::Lazy;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

const STACK_SIZE: usize = 4096 * 16; // 64 KiB

#[repr(align(4096), C)]
struct Stack {
    stack: [u8; STACK_SIZE],
}

impl Stack {
    const fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
        }
    }

    fn addr(&self) -> VirtAddr {
        VirtAddr::from_ptr(self.stack.as_ptr()) + STACK_SIZE as u64
    }
}

pub static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    static mut INTERRUPT_STACK_0: Stack = Stack::new();
    static mut INTERRUPT_STACK_1: Stack = Stack::new();
    static mut INTERRUPT_STACK_2: Stack = Stack::new();
    static mut INTERRUPT_STACK_3: Stack = Stack::new();
    static mut INTERRUPT_STACK_4: Stack = Stack::new();
    static mut INTERRUPT_STACK_5: Stack = Stack::new();
    static mut INTERRUPT_STACK_6: Stack = Stack::new();

    static mut PRIVILEGE_STACK_0: Stack = Stack::new();
    static mut PRIVILEGE_STACK_1: Stack = Stack::new();
    static mut PRIVILEGE_STACK_2: Stack = Stack::new();

    let mut tss = TaskStateSegment::new();

    unsafe {
        tss.interrupt_stack_table[0] = INTERRUPT_STACK_0.addr();
        tss.interrupt_stack_table[1] = INTERRUPT_STACK_1.addr();
        tss.interrupt_stack_table[2] = INTERRUPT_STACK_2.addr();
        tss.interrupt_stack_table[3] = INTERRUPT_STACK_3.addr();
        tss.interrupt_stack_table[4] = INTERRUPT_STACK_4.addr();
        tss.interrupt_stack_table[5] = INTERRUPT_STACK_5.addr();
        tss.interrupt_stack_table[6] = INTERRUPT_STACK_6.addr();

        tss.privilege_stack_table[0] = PRIVILEGE_STACK_0.addr();
        tss.privilege_stack_table[1] = PRIVILEGE_STACK_1.addr();
        tss.privilege_stack_table[2] = PRIVILEGE_STACK_2.addr();
    }

    tss
});
