use core::{fmt, marker::PhantomData};

use spin::Lazy;

use super::gdt::KERNEL_CODE_SELECTOR;
use crate::{address::VirtAddr, interrupts, registers::RFlags};

static IDTR: Lazy<IdtDescriptor> = Lazy::new(|| IdtDescriptor {
    size: core::mem::size_of_val(&*IDT) as u16 - 1,
    offset: VirtAddr::from_ptr(&*IDT),
});

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler(interrupts::breakpoint, 0);
    idt.page_fault.set_handler(interrupts::page_fault, 0);
    idt.double_fault.set_handler(interrupts::double_fault, 0);

    idt
});

pub fn init() {
    IDTR.load();
}

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: VirtAddr,
}
unsafe impl Send for IdtDescriptor {}
unsafe impl Sync for IdtDescriptor {}
impl IdtDescriptor {
    fn load(&self) {
        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) self);
        }
    }
}

#[repr(C)]
struct InterruptDescriptorTable {
    divide_error: Entry<HandlerFunc>,
    debug: Entry<HandlerFunc>,
    nmi: Entry<HandlerFunc>,
    breakpoint: Entry<HandlerFunc>,
    overflow: Entry<HandlerFunc>,
    bound_range_exceeded: Entry<HandlerFunc>,
    invalid_opcode: Entry<HandlerFunc>,
    device_not_available: Entry<HandlerFunc>,
    double_fault: Entry<HandlerFuncWithCode>,
    _coprocessor_segment_overrun: Entry<HandlerFunc>,
    invalid_tss: Entry<HandlerFuncWithCode>,
    segment_not_present: Entry<HandlerFuncWithCode>,
    stack_segment_fault: Entry<HandlerFuncWithCode>,
    general_protection_fault: Entry<HandlerFuncWithCode>,
    page_fault: Entry<HandlerFuncWithCode>,
    _reserved_1: Entry<HandlerFunc>,
    x87_floating_point: Entry<HandlerFunc>,
    alignment_check: Entry<HandlerFuncWithCode>,
    machine_check: Entry<HandlerFunc>,
    simd_floating_point: Entry<HandlerFunc>,
    virtualization: Entry<HandlerFunc>,
    control_protection: Entry<HandlerFuncWithCode>,
    _reserved_2: [Entry<HandlerFunc>; 10],
    user_defined: [Entry<HandlerFunc>; 256 - 32],
}
impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            divide_error: Entry::missing(),
            debug: Entry::missing(),
            nmi: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            _coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            _reserved_1: Entry::missing(),
            x87_floating_point: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point: Entry::missing(),
            virtualization: Entry::missing(),
            control_protection: Entry::missing(),
            _reserved_2: [Entry::missing(); 10],
            user_defined: [Entry::missing(); 256 - 32],
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Entry<F> {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    _reserved: u32,
    _phantom: PhantomData<F>,
}
impl<F> Entry<F>
where
    F: HandlerFuncTrait,
{
    const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            flags: 0,
            offset_mid: 0,
            offset_high: 0,
            _reserved: 0,
            _phantom: PhantomData,
        }
    }

    fn set_handler(&mut self, handler: F, ist: u8) {
        assert!(ist < 8);

        let offset = handler.addr().as_u64();

        self.offset_low = offset as u16;
        self.selector = KERNEL_CODE_SELECTOR;
        self.ist = ist;
        self.flags = 0b_1000_1110;
        self.offset_mid = (offset >> 16) as u16;
        self.offset_high = (offset >> 32) as u32;
    }
}

type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);
type HandlerFuncWithCode = extern "x86-interrupt" fn(InterruptStackFrame, u64);

trait HandlerFuncTrait {
    fn addr(self) -> VirtAddr;
}
impl HandlerFuncTrait for HandlerFunc {
    fn addr(self) -> VirtAddr {
        VirtAddr::from_ptr(self as *const ())
    }
}
impl HandlerFuncTrait for HandlerFuncWithCode {
    fn addr(self) -> VirtAddr {
        VirtAddr::from_ptr(self as *const ())
    }
}

#[repr(C)]
pub struct InterruptStackFrame {
    instruction_pointer: VirtAddr,
    code_segment: u16,
    _reserved1: [u8; 6],
    cpu_flags: RFlags,
    stack_pointer: VirtAddr,
    stack_segment: u16,
    _reserved2: [u8; 6],
}
impl fmt::Debug for InterruptStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InterruptStackFrame")
            .field("instruction_pointer", &self.instruction_pointer)
            .field("code_segment", &self.code_segment)
            .field("cpu_flags", &self.cpu_flags)
            .field("stack_pointer", &self.stack_pointer)
            .field("stack_segment", &self.stack_segment)
            .finish()
    }
}
