#![allow(dead_code)]
#![no_std]

pub mod boot;

// The only function you will need to implement
extern "Rust" {
    fn init();
    fn main();
}