#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
//
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

#[macro_use]
extern crate kernel;

use core::panic::PanicInfo;

use limine::{memory_map::EntryType, smp::Cpu};
use spin::Mutex;
use x86_64::instructions::{hlt, interrupts};

use kernel::{
    color::Color,
    console::CONSOLE,
    gdt, gsdata, hardware, idt, logger,
    ministack::create_ministack,
    process::{Manager, MANAGER},
    syscall, BASE_REVISION, SMP_RESPONSE,
};

kernel::entry!(main);

fn main() -> ! {
    logger::init();
    MANAGER.call_once(|| Mutex::new(Manager::init()));
    hardware::pics::init();

    for cpu in kernel::SMP_RESPONSE.cpus() {
        if cpu.id != 0 {
            log::info!("Starting CPU{}", cpu.id);
            cpu.goto_address.write(init_cpu);
        }
    }

    init_cpu(SMP_RESPONSE.cpus()[0]);
}

extern "C" fn init_cpu(cpu: &Cpu) -> ! {
    let cpuid = gsdata::CpuId::new(cpu.id);

    log::info!("{} started", cpuid);

    gdt::init(cpuid);
    idt::IDT.load();
    let lapic = hardware::lapic::init();
    syscall::init();

    log::info!("{} joining kernel", cpuid);
    let active_thread = MANAGER.get().unwrap().lock().join_kernel();

    // Setup core data
    let kernel_gs_data = gsdata::KernelData::new(
        cpuid,
        create_ministack(64 * 1024), // 64 KiB
        lapic,
        active_thread,
    );

    kernel_gs_data.as_ref().save_kernel_gsbase();

    if cpu.id == 0 {
        let mut manager = MANAGER.get().unwrap().lock();
        manager.spawn(include_bytes!("../app/hello"));
        manager.spawn(include_bytes!("../app/loop"));
        manager.spawn(include_bytes!("../app/yeild"));
        // manager.spawn(include_bytes!("../app/sleep"));
    }

    interrupts::enable();

    log::info!("{cpuid} Ready!");

    loop {
        log::info!("{} {:?}", cpuid, active_thread);
        println!("{} {:?}", cpuid, active_thread);

        hlt();
    }
}

#[panic_handler]
fn rust_panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    let mut console = CONSOLE.try_lock().unwrap_or_else(|| {
        unsafe { CONSOLE.force_unlock() };
        CONSOLE.lock()
    });
    console.set_colors(Color::WHITE, Color::rgb(0, 0, 128));
    print!("{}", info);

    kernel::shutdown_emu();
}
