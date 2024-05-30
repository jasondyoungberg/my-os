use core::fmt;

use crate::address::PhysAddr;

use super::{Response, MAGIC_1, MAGIC_2};

#[repr(C)]
pub struct MemoryMapRequest {
    id: [u64; 4],
    revision: u64,
    pub response: Response<MemoryMapResponse>,
}
impl MemoryMapRequest {
    pub const fn new() -> Self {
        Self {
            id: [MAGIC_1, MAGIC_2, 0x67cf3d9d378a806f, 0xe304acdfc50c3c62],
            revision: 0,
            response: Response::none(),
        }
    }
}
#[repr(C)]
pub struct MemoryMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const MemoryMapEntry,
}
unsafe impl Sync for MemoryMapResponse {}
unsafe impl Send for MemoryMapResponse {}
impl MemoryMapResponse {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn entries(&self) -> impl Iterator<Item = &MemoryMapEntry> {
        (unsafe { core::slice::from_raw_parts(self.entries, self.entry_count as usize) })
            .iter()
            .map(|&fb| unsafe { &*fb })
    }
}
impl fmt::Debug for MemoryMapResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.entries()).finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct MemoryMapEntry {
    pub base: PhysAddr,
    pub length: u64,
    pub entry_type: MemoryMapEntryType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum MemoryMapEntryType {
    Usable = 0,
    Reserved = 1,
    AcpiReclaimable = 2,
    AcpiNvs = 3,
    BadMemory = 4,
    BootloaderReclaimable = 5,
    KernelAndModules = 6,
    Framebuffer = 7,
}
