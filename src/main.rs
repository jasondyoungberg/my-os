use std::path::PathBuf;
use std::process::Command;

fn main() {
    std::fs::create_dir_all("./build").unwrap();

    let config = get_config();
    let img_path = build(&config);

    let mut cmd = Command::new("qemu-system-x86_64");

    match config.sys_type {
        SysType::Bios => {}
        SysType::Uefi => {
            cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        }
    }

    match config.mode {
        Mode::Normal => {}
        Mode::Debug => {
            cmd.args(["-d", "int,cpu_reset,unimp,guest_errors"]);
        }
        Mode::Test => {
            cmd.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"])
                .args(["-display", "none"]);
        }
    }

    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", img_path.display()));

    cmd.args(["-m", "4G"])
        .args(["-display", "sdl"])
        .args(["-nodefaults"])
        .args(["-vga", "std"])
        .args(["-debugcon", "stdio"])
        .arg("--no-reboot")
        .arg("--no-shutdown");

    std::process::exit(cmd.spawn().unwrap().wait().unwrap().code().unwrap());
}

fn build(config: &Config) -> PathBuf {
    let out_dir = PathBuf::from("./build");
    let img_path = out_dir.join("os.img");

    match config.sys_type {
        SysType::Uefi => {
            bootloader::UefiBoot::new(&config.kernel)
                .create_disk_image(&img_path)
                .unwrap();
        }
        SysType::Bios => {
            bootloader::BiosBoot::new(&config.kernel)
                .create_disk_image(&img_path)
                .unwrap();
        }
    }

    img_path
}

fn get_config() -> Config {
    let mut kernel = PathBuf::from(env!("KERNEL_PATH"));
    let mut sys_type = SysType::Bios;
    let mut mode = Mode::Normal;

    let mut args = std::env::args();
    args.next(); // skip the first argument

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--kernel" => {
                kernel = PathBuf::from(args.next().expect("no kernel provided"));
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
                println!("Usage: cargo run -- [options]");
                println!("Options:");
                println!("  --kernel <path>  Path to the kernel binary");
                println!("  --test           Run in test mode");
                println!("  --debug          Enable QEMU debug output");
                println!("  --uefi           Use UEFI firmware");
                println!("  --bios           Use BIOS firmware");
                println!("  --help           Print this help message");
                std::process::exit(0);
            }
            _ => {
                panic!("Unknown argument: {arg}");
            }
        }
    }

    Config {
        kernel,
        sys_type,
        mode,
    }
}

#[derive(Debug)]
struct Config {
    kernel: PathBuf,
    sys_type: SysType,
    mode: Mode,
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
