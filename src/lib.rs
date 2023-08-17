#![allow(dead_code)]
#![no_std]

pub mod boot;

// The only function you will need to implements
extern "Rust" {
    fn init();
    fn main();
}