use crate::dbg;
use core::{future::poll_fn, task::Poll};
use futures::task::AtomicWaker;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};

static WAKER: AtomicWaker = AtomicWaker::new();

pub fn wake() {
    WAKER.wake();
}

pub struct AtaDisk {
    ports: Ports,
}

// https://wiki.osdev.org/ATA_read/write_sectors#Read_in_LBA_mode

impl AtaDisk {
    pub const unsafe fn new(base: u16) -> Self {
        Self {
            ports: Ports::new(base),
        }
    }

    pub async fn read_sectors(&mut self, lba: u32, sectors: u8, buffer: &mut [u8]) {
        self.set_rw_address(lba, sectors);
        self.send_commannd(Command::ReadSectorsWithRetry);
        self.wait_for_data_request().await;

        for i in 0..(256 * sectors as usize) {
            let data = unsafe { self.ports.data.read() };
            let bytes = data.to_le_bytes();
            buffer[2 * i] = bytes[0];
            buffer[2 * i + 1] = bytes[1];
        }
    }

    pub async fn write_sectors(&mut self, lba: u32, sectors: u8, buffer: &[u8]) {
        self.set_rw_address(lba, sectors);
        self.send_commannd(Command::WriteSectorsWithRetry);
        self.wait_for_data_request().await;

        for i in 0..(256 * sectors as usize) {
            let bytes = [buffer[2 * i], buffer[2 * i + 1]];
            let data = u16::from_le_bytes(bytes);
            unsafe { self.ports.data.write(data) };
        }
    }

    fn set_rw_address(&mut self, lba: u32, sectors: u8) {
        assert!(lba < 0x_1000_0000, "LBA must be less that 2^28");

        let lba_bytes = lba.to_le_bytes();

        unsafe {
            self.ports.device.write(lba_bytes[3] | 0b_1110_0000);
            self.ports.sector_count.write(sectors);
            self.ports.lba_low.write(lba_bytes[0]);
            self.ports.lba_mid.write(lba_bytes[1]);
            self.ports.lba_high.write(lba_bytes[2]);
        }
    }

    fn send_commannd(&mut self, command: Command) {
        unsafe {
            self.ports.command.write(command as u8);
        }
    }

    fn check_status(&mut self) -> Status {
        unsafe { self.ports.status.read() }.into()
    }

    async fn wait_for_data_request(&mut self) {
        poll_fn(|cx| {
            dbg!("poll disk");

            if self.check_status().data_request {
                return Poll::Ready(());
            }

            WAKER.register(cx.waker());

            if self.check_status().data_request {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

// todo: maybe there's a better way to do this
#[allow(clippy::struct_excessive_bools)]
struct Status {
    error: bool,
    index: bool,
    corrected_data: bool,
    data_request: bool,
    service: bool,
    drive_fault: bool,
    ready: bool,
    busy: bool,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self {
            error: value & 0b_0000_0001 != 0,
            index: value & 0b_0000_0010 != 0,
            corrected_data: value & 0b_0000_0100 != 0,
            data_request: value & 0b_0000_1000 != 0,
            service: value & 0b_0001_0000 != 0,
            drive_fault: value & 0b_0010_0000 != 0,
            ready: value & 0b_0100_0000 != 0,
            busy: value & 0b_1000_0000 != 0,
        }
    }
}

#[repr(u8)]
enum Command {
    ReadSectorsWithRetry = 0x20,
    WriteSectorsWithRetry = 0x30,
}

struct Ports {
    data: Port<u16>,
    error: PortReadOnly<u8>,
    features: PortWriteOnly<u8>,
    sector_count: Port<u8>,
    lba_low: Port<u8>,
    lba_mid: Port<u8>,
    lba_high: Port<u8>,
    device: Port<u8>,
    status: PortReadOnly<u8>,
    command: PortWriteOnly<u8>,
}

impl Ports {
    const fn new(base: u16) -> Self {
        Self {
            data: Port::new(base),
            error: PortReadOnly::new(base + 1),
            features: PortWriteOnly::new(base + 1),
            sector_count: Port::new(base + 2),
            lba_low: Port::new(base + 3),
            lba_mid: Port::new(base + 4),
            lba_high: Port::new(base + 5),
            device: Port::new(base + 6),
            status: PortReadOnly::new(base + 7),
            command: PortWriteOnly::new(base + 7),
        }
    }
}
