#![no_std]
#![no_main]

#[macro_use]
extern crate stdlib;

entry!(main);
fn main() {
    println!("Hello, world! I'm a Rust program in userspace!");
    stdlib::yeild().unwrap();
    println!("I can yeild and come back!");
    eprintln!("And I can write to stderr too!");
}
