use core::ops::Range;

use spin::Mutex;
use x86_64::instructions::port::Port;

pub const PIC_OFFSET: u8 = 0x20;
pub const PIC_RANGE_START: u8 = PIC_OFFSET;
pub const PIC_RANGE_END: u8 = PIC_OFFSET + 15;

pub static PIC: Mutex<Pic> = Mutex::new(Pic::new());

pub struct Pic {
    master_command: Port<u8>,
    master_data: Port<u8>,
    slave_command: Port<u8>,
    slave_data: Port<u8>,
}

impl Pic {
    const fn new() -> Self {
        Self {
            master_command: Port::new(0x20),
            master_data: Port::new(0x21),
            slave_command: Port::new(0xA0),
            slave_data: Port::new(0xA1),
        }
    }

    pub fn init(&mut self) {
        // https://wiki.osdev.org/8259_PIC
        unsafe {
            self.master_command.write(0x11);
            io_wait();
            self.slave_command.write(0x11);
            io_wait();
            self.master_data.write(PIC_OFFSET);
            io_wait();
            self.slave_data.write(PIC_OFFSET + 8);
            io_wait();
            self.master_data.write(4);
            io_wait();
            self.slave_data.write(2);
            io_wait();
            self.master_data.write(1);
            io_wait();
            self.slave_data.write(1);
            io_wait();
            self.master_data.write(0xff);
            io_wait();
            self.slave_data.write(0xff);
            io_wait();
        }
    }
}

fn io_wait() {
    unsafe { Port::new(0x80).write(0u8) };
}
