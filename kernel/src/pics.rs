use pic8259::ChainedPics;
use spin::Mutex;

pub const PICS_OFFSET: u8 = 32;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new_contiguous(PICS_OFFSET) });

pub fn init() {
    let mut pics = PICS.lock();
    unsafe {
        pics.initialize();
        pics.write_masks(0, 0);
    }
}
