#![no_std]
#![no_main]

#[macro_use]
extern crate stdlib;

entry!(main);
fn main() {
    stdlib::write(1, b"Hello, world! I'm (barely) a rust program!\n").unwrap();
    // println!("Hello, world!");
}
