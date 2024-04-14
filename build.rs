use std::{env, path::PathBuf};

use bootloader::DiskImageBuilder;

fn main() {
    const BIN_PREFIX: &str = "CARGO_BIN_FILE_KERNEL_";

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("no OUT_DIR env var"));

    for (key, value) in env::vars() {
        if !key.starts_with(BIN_PREFIX) {
            continue;
        }

        let name = key[BIN_PREFIX.len()..].to_string();

        let kernel_path = PathBuf::from(value);

        let bios_path = out_dir.join(format!("{}-bios.img", name));
        let uefi_path = out_dir.join(format!("{}-uefi.img", name));

        let image = DiskImageBuilder::new(kernel_path.clone());

        image
            .create_bios_image(&bios_path)
            .expect("failed to create BIOS image");
        image
            .create_uefi_image(&uefi_path)
            .expect("failed to create UEFI image");

        println!("cargo:rustc-env=IMG_{}_BIOS={}", name, bios_path.display());
        println!("cargo:rustc-env=IMG_{}_UEFI={}", name, uefi_path.display());
    }
}
