#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    unused_unsafe,
    clippy::missing_safety_doc,
    clippy::multiple_unsafe_ops_per_block,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used
)]
#![allow(
    clippy::cast_possible_truncation, // Only x86_64 is supported
    clippy::missing_panics_doc // This crate should never panic on recoverable errors
)]

pub mod print;
