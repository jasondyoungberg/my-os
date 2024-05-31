use core::{
    ffi::{c_char, CStr},
    slice,
};

use bitflags::bitflags;

use super::{Response, MAGIC_1, MAGIC_2};

#[repr(C)]
pub struct ModuleRequest {
    id: [u64; 4],
    revision: u64,
    pub response: Response<ModuleResponse>,
    internel_module_count: u64,
    internal_modules: *const *const InternalModule,
}
unsafe impl Sync for ModuleRequest {}
unsafe impl Send for ModuleRequest {}
impl ModuleRequest {
    pub const fn new(modules: &'static [&'static InternalModule]) -> Self {
        Self {
            id: [MAGIC_1, MAGIC_2, 0x3e7e279702be32af, 0xca1c4f3bd1280cee],
            revision: 0,
            response: Response::none(),
            internel_module_count: modules.len() as u64,
            internal_modules: modules.as_ptr().cast(),
        }
    }
}

#[repr(C)]
pub struct InternalModule {
    path: *const c_char,
    cmdline: *const c_char,
    flags: InternalModuleFlags,
}

bitflags! {
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct InternalModuleFlags: u64 {
        const REQUIRED = 1 << 0;
        const COMPRESSED = 1 << 1;
    }
}

#[repr(C)]
pub struct ModuleResponse {
    revision: u64,
    module_count: u64,
    modules: *const *const File,
}
unsafe impl Sync for ModuleResponse {}
unsafe impl Send for ModuleResponse {}
impl ModuleResponse {
    pub fn modules(&self) -> &[&File] {
        unsafe { core::slice::from_raw_parts(self.modules.cast(), self.module_count as usize) }
    }
}

#[repr(C)]
pub struct File {
    revision: u64,
    address: *const (),
    size: u64,
    path: *const c_char,
    cmd: *const c_char,
    pub media_type: MediaType,
    _unused: u32,
    tftp_ip: u32,
    tftp_port: u32,
    partition_index: u32,
    mbr_disk_id: u32,
}
impl File {
    pub fn slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.address.cast(), self.size as usize) }
    }
    pub fn path(&self) -> &str {
        unsafe { CStr::from_ptr(self.path).to_str().unwrap() }
    }
    pub fn cmdline(&self) -> &str {
        unsafe { CStr::from_ptr(self.cmd).to_str().unwrap() }
    }
}

#[repr(C)]
pub struct Uuid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

#[repr(u32)]
pub enum MediaType {
    Generic = 0,
    Optical = 1,
    Tftp = 2,
}
