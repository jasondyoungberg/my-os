use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use fatfs::FileSystem;

const SIZE: u64 = 1024 * 1024 * 1024; // 1GB

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR not set"));
    let kernel_path = PathBuf::from(
        std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel")
            .expect("CARGO_BIN_FILE_KERNEL_kernel not set"),
    );
    let kernel = fs::read(&kernel_path).unwrap();

    // Create an empty disk image
    let img_path = out_dir.join("disk.img");
    let img_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(&img_path)
        .expect("failed to create disk image");
    img_file
        .set_len(SIZE)
        .expect("failed to set disk image size");

    let mut image = Image::new(&img_file);

    image.create_file(PathBuf::from("EFI/Boot/BootX64.efi"), &kernel);

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=IMG_PATH={}", img_path.display());
    println!("cargo:rustc-env=KERNEL_PATH={}", kernel_path.display());
}

struct Image<'a>(FileSystem<&'a File>);

impl<'a> Image<'a> {
    fn new(file: &'a File) -> Self {
        fatfs::format_volume(file, fatfs::FormatVolumeOptions::new())
            .expect("failed to format disk image");

        let fs = fatfs::FileSystem::new(file, fatfs::FsOptions::new())
            .expect("failed to load file system");

        Self(fs)
    }

    fn create_file(&mut self, path: PathBuf, content: &[u8]) {
        let root_dir = self.0.root_dir();

        let filename = path
            .file_name()
            .expect("no filename")
            .to_str()
            .expect("invalid filename");

        let dir = path
            .parent()
            .expect("Unable to get parent dir")
            .iter()
            .fold(root_dir, |dir, part| {
                dir.create_dir(part.to_str().expect("invalid dir name"))
                    .expect("failed to create dir")
            });

        dir.create_file(filename)
            .expect("failed to create file")
            .write_all(content)
            .expect("failed to write to file");
    }
}
