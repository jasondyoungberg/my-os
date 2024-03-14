fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let mut cmd = std::process::Command::new("qemu-system-x86_64");

    let uefi = std::env::args().any(|arg| arg == "uefi");

    if uefi {
        cmd.arg("-bios")
            .arg(ovmf_prebuilt::ovmf_pure_efi())
            .arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }

    cmd.args(["-m", "4G"])
        .args(["-display", "sdl"])
        .args(["-nodefaults"])
        .args(["-vga", "std"])
        .args(["-debugcon", "stdio"])
        .arg("--no-reboot")
        .arg("--no-shutdown");

    cmd.spawn().unwrap().wait().unwrap();
}
