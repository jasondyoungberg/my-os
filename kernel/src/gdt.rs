pub const KERNEL_CODE_SELECTOR: u16 = 1 << 3;
pub const KERNEL_DATA_SELECTOR: u16 = 2 << 3;
// pub const USER_DATA_SELECTOR: u16 = 3 << 3;
// pub const USER_CODE_SELECTOR: u16 = 4 << 3;

static GDT: GlobalDescriptorTable = GlobalDescriptorTable {
    entries: [
        GdtEntry::null(),
        GdtEntry::kernel_code(),
        GdtEntry::kernel_data(),
    ],
};

static GDTR: GdtDescriptor = GdtDescriptor {
    size: core::mem::size_of::<GlobalDescriptorTable>() as u16 - 1,
    offset: &GDT,
};

pub fn init() {
    GDTR.load();
    unsafe {
        core::arch::asm!("
            mov ss, {data}

            // set up code segment
            push {code}
            lea {tmp}, [1f + rip]
            push {tmp}
            retfq
            1:
        ",
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
    offset: *const GlobalDescriptorTable,
}
unsafe impl Send for GdtDescriptor {}
unsafe impl Sync for GdtDescriptor {}
impl GdtDescriptor {
    fn load(&self) {
        unsafe { core::arch::asm!("lgdt [{0}]", in(reg) self) }
    }
}

#[repr(C)]
struct GlobalDescriptorTable {
    entries: [GdtEntry; 3],
}

#[repr(C)]
struct GdtEntry {
    _unused1: u32,
    _unused2: u8,
    access: u8,
    flags: u8,
    _unused3: u8,
}
impl GdtEntry {
    const fn null() -> Self {
        Self {
            _unused1: 0,
            _unused2: 0,
            access: 0,
            flags: 0,
            _unused3: 0,
        }
    }

    const fn kernel_code() -> Self {
        Self {
            _unused1: 0,
            _unused2: 0,
            access: 0b_1001_1011, // present, ring 0, code, readable, accessed
            flags: 0b_0010 << 4,  // 64-bit
            _unused3: 0,
        }
    }

    const fn kernel_data() -> Self {
        Self {
            _unused1: 0,
            _unused2: 0,
            access: 0b_1001_0011, // present, ring 0, data, writable, accessed
            flags: 0b_0010 << 4,  // 64-bit
            _unused3: 0,
        }
    }
}
