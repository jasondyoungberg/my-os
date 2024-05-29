#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
//
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

extern crate alloc;

pub mod color;
pub mod console;
pub mod debug;
pub mod exception;
pub mod gdt;
pub mod gsdata;
pub mod hardware;
pub mod heap;
pub mod idt;
pub mod logger;
pub mod macros;
pub mod mapper;
pub mod memory;
pub mod process;
pub mod syscall;

use spin::Lazy;

#[used]
pub static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
pub static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest =
    limine::request::FramebufferRequest::new();
pub static FRAMEBUFFER_RESPONSE: Lazy<&limine::response::FramebufferResponse> =
    Lazy::new(|| FRAMEBUFFER_REQUEST.get_response().unwrap());

#[used]
pub static MEMORY_MAP_REQUEST: limine::request::MemoryMapRequest =
    limine::request::MemoryMapRequest::new();
pub static MEMORY_MAP_RESPONSE: Lazy<&limine::response::MemoryMapResponse> =
    Lazy::new(|| MEMORY_MAP_REQUEST.get_response().unwrap());

#[used]
pub static SMP_REQUEST: limine::request::SmpRequest = limine::request::SmpRequest::new();
pub static SMP_RESPONSE: Lazy<&limine::response::SmpResponse> =
    Lazy::new(|| SMP_REQUEST.get_response().unwrap());

#[used]
pub static HHDM_REQUEST: limine::request::HhdmRequest = limine::request::HhdmRequest::new();
pub static HHDM_RESPONSE: Lazy<&limine::response::HhdmResponse> =
    Lazy::new(|| HHDM_REQUEST.get_response().unwrap());

#[used]
pub static STACK_SIZE_REQUEST: limine::request::StackSizeRequest =
    limine::request::StackSizeRequest::new().with_size(1024 * 1024); // 1 MiB
pub static STACK_SIZE_RESPONSE: Lazy<&limine::response::StackSizeResponse> =
    Lazy::new(|| STACK_SIZE_REQUEST.get_response().unwrap());

#[used]
pub static MODULE_REQUEST: limine::request::ModuleRequest = limine::request::ModuleRequest::new()
    .with_internal_modules(&[
        &limine::modules::InternalModule::new().with_path(limine::cstr!("/hello.txt"))
    ]);
pub static MODULE_RESPONSE: Lazy<&limine::response::ModuleResponse> =
    Lazy::new(|| MODULE_REQUEST.get_response().unwrap());

pub fn find_file(path: &str) -> &'static limine::file::File {
    MODULE_RESPONSE
        .modules()
        .iter()
        .find(|f| f.path() == path.as_bytes())
        .expect("module not found")
}

pub fn read_file(file: &'static limine::file::File) -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(file.addr(), file.size() as usize) }
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        const _: fn() -> ! = $path;

        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            assert!(kernel::BASE_REVISION.is_supported());
            assert!(kernel::FRAMEBUFFER_REQUEST.get_response().is_some());
            assert!(kernel::MEMORY_MAP_REQUEST.get_response().is_some());
            assert!(kernel::SMP_REQUEST.get_response().is_some());
            assert!(kernel::HHDM_REQUEST.get_response().is_some());
            assert!(kernel::STACK_SIZE_REQUEST.get_response().is_some());
            $path()
        }
    };
}

pub fn shutdown_emu() {
    use x86_64::instructions;

    unsafe { instructions::port::PortWriteOnly::<u16>::new(0x604).write(0x2000) };
}
