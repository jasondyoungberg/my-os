use core::mem::size_of;

use alloc::{boxed::Box, vec};
use bitflags::bitflags;
use spin::Lazy;

pub const KERNEL_CODE_SELECTOR: u16 = 1 << 3;
pub const KERNEL_DATA_SELECTOR: u16 = 2 << 3;
pub const USER_DATA_SELECTOR: u16 = 3 << 3 | 3;
pub const USER_CODE_SELECTOR: u16 = 4 << 3 | 3;

static GDT: Lazy<GdtInfo> = Lazy::new(|| {
    let gdt = vec![
        GdtEntry::null(),
        GdtEntry::kernel_code(),
        GdtEntry::kernel_data(),
        GdtEntry::user_data(),
        GdtEntry::user_code(),
    ];

    let gdtr = GdtDescriptor {
        size: (size_of::<GdtEntry>() * gdt.len()) as u16 - 1,
        offset: gdt.leak().as_ptr(),
    };
    let gdtr = Box::into_raw(Box::new(gdtr));

    GdtInfo { gdtr }
});

struct GdtInfo {
    gdtr: *const GdtDescriptor,
}
unsafe impl Send for GdtInfo {}
unsafe impl Sync for GdtInfo {}

pub fn init() {
    unsafe {
        core::arch::asm!("
            lgdt [{gdtr}]
            mov ss, {data}

            // set up code segment
            push {code}
            lea {tmp}, [1f + rip]
            push {tmp}
            retfq
            1:
        ",
            gdtr = in(reg) GDT.gdtr,
            code = in(reg) KERNEL_CODE_SELECTOR as u64,
            data = in(reg) KERNEL_DATA_SELECTOR as u64,
            tmp = lateout(reg) _,
            options(preserves_flags)

        )
    }
}

#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: *const GdtEntry,
}
unsafe impl Send for GdtDescriptor {}
unsafe impl Sync for GdtDescriptor {}
impl GdtDescriptor {
    fn load(&self) {
        unsafe { core::arch::asm!("lgdt [{0}]", in(reg) self) }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct GdtFlags: u64 {
        const ACCESSED =    1 << 40;
        const READ_WRITE =  1 << 41;
        const CONFORMING =  1 << 42;
        const EXECUTABLE =  1 << 43;
        const CODE_DATA =   1 << 44;
        const DPL_RING3 =   3 << 45;
        const PRESENT =     1 << 47;
        const LONG_MODE =   1 << 53;
        const SIZE =        1 << 54;
        const GRANULARITY = 1 << 55;
    }
}

#[repr(C)]
struct GdtEntry(u64);
impl GdtEntry {
    const fn null() -> Self {
        Self(0)
    }

    fn kernel_code() -> Self {
        Self(
            (GdtFlags::ACCESSED
                | GdtFlags::READ_WRITE
                | GdtFlags::EXECUTABLE
                | GdtFlags::CODE_DATA
                | GdtFlags::PRESENT
                | GdtFlags::LONG_MODE)
                .bits(),
        )
    }

    fn kernel_data() -> Self {
        Self(
            (GdtFlags::ACCESSED
                | GdtFlags::READ_WRITE
                | GdtFlags::CODE_DATA
                | GdtFlags::PRESENT
                | GdtFlags::LONG_MODE)
                .bits(),
        )
    }

    fn user_code() -> Self {
        Self(
            (GdtFlags::ACCESSED
                | GdtFlags::READ_WRITE
                | GdtFlags::EXECUTABLE
                | GdtFlags::CODE_DATA
                | GdtFlags::DPL_RING3
                | GdtFlags::PRESENT
                | GdtFlags::LONG_MODE)
                .bits(),
        )
    }

    fn user_data() -> Self {
        Self(
            (GdtFlags::ACCESSED
                | GdtFlags::READ_WRITE
                | GdtFlags::CODE_DATA
                | GdtFlags::DPL_RING3
                | GdtFlags::PRESENT
                | GdtFlags::LONG_MODE)
                .bits(),
        )
    }
}
