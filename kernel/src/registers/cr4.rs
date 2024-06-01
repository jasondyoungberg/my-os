bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default)]
    #[repr(transparent)]
    pub struct Cr4Flags: u64 {
        const VIRRTUAL_8086_MODE_EXTENSIONS = 1 << 0;
        const PROTECTED_MODE_VIRTUAL_INTERRUPTS = 1 << 1;
        const TIME_STAMP_DISABLE = 1 << 2;
        const DEBUGGING_EXTENSIONS = 1 << 3;
        const PAGE_SIZE_EXTENSIONS = 1 << 4;
        const PHYSICAL_ADDRESS_EXTENSIONS = 1 << 5;
        const MACHINE_CHECK_EXCEPTION = 1 << 6;
        const PAGE_GLOBAL_ENABLE = 1 << 7;
        const PERFORMANCE_MONITORING_COUNTER_ENABLE = 1 << 8;
        const OPERATING_SYSTEM_SUPPORT_FOR_FXSAVE_AND_FXRSTOR_INSTRUCTIONS = 1 << 9;
        const OS_SUPPORT_FOR_UNMASKED_SIMD_EXCEPTIONS = 1 << 10;
        const USER_MODE_INSTRUCTION_PREVENTION = 1 << 11;
        const FIVE_LEVEL_PAGING = 1 << 12;
        const VMX_ENABLE = 1 << 13;
        const SMX_ENABLE = 1 << 14;
        const FSGSBASE_ENABLE = 1 << 16;
        const PCID_ENABLE = 1 << 17;
        const XSAVE_ENABLE = 1 << 18;
        const KEY_LOCKER_ENABLE = 1 << 19;
        const SMEP_ENABLE = 1 << 20;
        const SMAP_ENABLE = 1 << 21;
        const PROTECTION_KEYS_USER = 1 << 22;
        const CONTROL_FLOW_ENFORCEMENT = 1 << 23;
        const PROTECTION_KEYS_SUPERVISOR = 1 << 24;
        const USER_INTERRUPTS = 1 << 25;
    }
}

pub struct Cr4;
impl Cr4 {
    pub fn read() -> Cr4Flags {
        let flags: u64;
        unsafe { core::arch::asm!("mov {}, cr4", out(reg) flags, options(nomem, preserves_flags)) };
        Cr4Flags::from_bits_retain(flags)
    }

    pub unsafe fn write(flags: Cr4Flags) {
        unsafe { core::arch::asm!("mov cr4, {}", in(reg) flags.bits(), options(nomem)) };
    }
}
