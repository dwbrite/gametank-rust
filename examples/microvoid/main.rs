#![no_std]
#![no_main]

extern crate gt_crust;

pub mod system;
mod font;

use core::{iter, mem};
use core::iter::once;
use crate::system::scr::*;
use crate::system::bcr::*;
use crate::system::console::*;

#[no_mangle]
fn main() {
    let mut console = Console::init();



    loop {
        console.await_vblank();
        console.flip_framebuffer();

        console.draw_box(0, 0, 127, 127, 0b010_00_100);
        console.draw_box(0, 100, 127, 27, 0b111_10_101);
    }
}