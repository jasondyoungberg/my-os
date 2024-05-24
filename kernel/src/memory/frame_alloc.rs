
use bootloader_api::info::{MemoryRegion, MemoryRegionKind, MemoryRegions};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

pub struct BootInfoFrameAllocator {
    regions: [Option<MemoryRegion>; 16],
    next: usize,
}

impl BootInfoFrameAllocator {
    pub fn new(all_regions: &'static MemoryRegions) -> Self {
        let mut regions = [None; 16];

        all_regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            .enumerate()
            .for_each(|(i, &r)| regions[i] = Some(r));

        Self { regions, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        self.regions
            .iter()
            .filter_map(|&r| r)
            .map(|r| r.start..r.end)
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
