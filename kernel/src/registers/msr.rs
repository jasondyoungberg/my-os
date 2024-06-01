use core::arch::asm;

pub struct ModelSpecificRegister<const ADDR: u32>();

impl<const ADDR: u32> ModelSpecificRegister<ADDR> {
    pub fn read() -> u64 {
        let value_high: u32;
        let value_low: u32;
        unsafe {
            asm!("rdmsr", in("ecx") ADDR, out("edx") value_high, out("eax") value_low);
        }
        ((value_high as u64) << 32) | (value_low as u64)
    }

    pub fn write(value: u64) {
        let value_high = (value >> 32) as u32;
        let value_low = value as u32;
        unsafe {
            asm!("wrmsr", in("ecx") ADDR, in("edx") value_high, in("eax") value_low);
        }
    }
}
