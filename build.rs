fn main() {
    // Pass the kernel binary to main.rs
    let kernel = std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap();
    println!("cargo::rustc-env=KERNEL_PATH={}", kernel.to_str().unwrap());
}
