use std::{env, fs, path::PathBuf};

use bootloader::DiskImageBuilder;

fn main() {
    const BIN_PREFIX: &str = "CARGO_BIN_FILE_KERNEL_";
    const APP_PREFIX: &str = "CARGO_BIN_FILE_APP_";

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("no OUT_DIR env var"));

    for (key, value) in env::vars() {
        if !key.starts_with(BIN_PREFIX) {
            continue;
        }

        let name = key[BIN_PREFIX.len()..].to_string();

        let kernel_path = PathBuf::from(value);

        let bios_path = out_dir.join(format!("{}-bios.img", name));
        let uefi_path = out_dir.join(format!("{}-uefi.img", name));

        let kernel_size = fs::metadata(&kernel_path)
            .expect("failed to get kernel size")
            .len();

        let mut image = DiskImageBuilder::new(kernel_path.clone());

        let files_path = PathBuf::from("./files/");

        for path in fs::read_dir(files_path.clone()).expect("failed to read files dir") {
            let entry = path.expect("failed to read file");
            let full_path = entry.path();
            let rel_path = full_path.strip_prefix(&files_path).unwrap();
            image.set_file(rel_path.to_str().unwrap().to_string(), full_path);
        }

        for (app_key, app_value) in env::vars() {
            if !app_key.starts_with(APP_PREFIX) {
                continue;
            }

            let app_name = app_key
                .chars()
                .skip_while(|c| c.is_uppercase() || *c == '_')
                .skip("app_".len())
                .collect::<String>();

            if app_name.is_empty() {
                continue;
            }

            let app_name = format!("{app_name}.app");

            let app_path = PathBuf::from(app_value);

            image.set_file(app_name, app_path);
        }

        image
            .create_bios_image(&bios_path)
            .expect("failed to create BIOS image");
        image
            .create_uefi_image(&uefi_path)
            .expect("failed to create UEFI image");

        println!("cargo:rustc-env=IMG_{}_BIOS={}", name, bios_path.display());
        println!("cargo:rustc-env=IMG_{}_UEFI={}", name, uefi_path.display());
        println!("cargo:rustc-env=KERNEL_SIZE_{}={}", name, kernel_size);
    }
}
