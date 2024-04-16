use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let config = get_config();
    let img_path = config.img_path;

    let mut cmd = Command::new("qemu-system-x86_64");

    match config.sys_type {
        SysType::Bios => {}
        SysType::Uefi => {
            cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        }
    }

    match config.mode {
        Mode::Normal => {
            cmd.args(["-display", "sdl"]);
        }
        Mode::Debug => {
            cmd.args(["-display", "sdl"])
                .args(["-d", "int,cpu_reset,unimp,guest_errors"]);
        }
        Mode::Test => {
            cmd.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"])
                .args(["-display", "none"]);
        }
    }

    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", img_path.display()));

    cmd.args(["-machine", "pc"])
        .args(["-m", "4G"])
        .args(["-nodefaults"])
        .args(["-vga", "std"])
        .args(["-debugcon", "stdio"])
        .arg("--no-reboot")
        .arg("--no-shutdown");

    for arg in config.qemu_args {
        cmd.arg(arg);
    }

    println!("Running QEMU with image: {}", img_path.display());

    cmd.spawn().unwrap().wait().unwrap();
}

fn get_config() -> Config {
    let mut kernel = "kernel".to_string();
    let mut sys_type = SysType::Bios;
    let mut mode = Mode::Normal;
    let mut qemu_args = Vec::new();

    let mut args = std::env::args();
    args.next(); // skip the first argument

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--kernel" => {
                kernel = args.next().expect("no kernel provided");
            }
            "--test" => mode = Mode::Test,
            "--debug" => mode = Mode::Debug,

            "--uefi" => {
                sys_type = SysType::Uefi;
            }
            "--bios" => {
                sys_type = SysType::Bios;
            }
            "--help" => {
                println!("Usage: cargo run -- [options] [-- qemu args]");
                println!("Options:");
                println!("  --kernel <path>  Kernel to use");
                println!("  --test           Run in test mode");
                println!("  --debug          Enable QEMU debug output");
                println!("  --uefi           Use UEFI firmware");
                println!("  --bios           Use BIOS firmware");
                println!("  --help           Print this help message");
                std::process::exit(0);
            }
            "--" => {
                qemu_args = args.collect();
                break;
            }
            _ => {
                panic!("Unknown argument: {arg}");
            }
        }
    }
    let img_path = PathBuf::from(
        env::var_os(format!("IMG_{kernel}_{sys_type}")).expect("kernel image not found"),
    );

    println!(
        "Kernel size: {} kB",
        env::var(format!("KERNEL_SIZE_{kernel}"))
            .unwrap()
            .parse::<usize>()
            .unwrap()
            / 1024
    );

    Config {
        img_path,
        sys_type,
        mode,
        qemu_args,
    }
}

#[derive(Debug)]
struct Config {
    img_path: PathBuf,
    sys_type: SysType,
    mode: Mode,
    qemu_args: Vec<String>,
}

#[derive(Debug)]
enum SysType {
    Uefi,
    Bios,
}

#[derive(Debug)]
enum Mode {
    Normal,
    Debug,
    Test,
}

impl std::fmt::Display for SysType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SysType::Uefi => write!(f, "UEFI"),
            SysType::Bios => write!(f, "BIOS"),
        }
    }
}
