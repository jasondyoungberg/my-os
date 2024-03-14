fn main() {
    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive")
        .arg(format!("file={},format=raw", env!("IMG_PATH")));

    cmd.arg("-nodefaults");
    cmd.arg("-vga").arg("std");
    cmd.arg("-display").arg("sdl");

    cmd.spawn()
        .expect("failed to start qemu")
        .wait()
        .expect("qemu failed");
}
