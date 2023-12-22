#![no_std]
#![no_main]

extern crate gt_crust;

pub mod system;
mod font;

use core::{iter, mem};
use core::iter::once;
use dgtf_macros::string_to_indices;
use crate::system::scr::*;
use crate::system::bcr::*;
use crate::system::console::*;

#[no_mangle]
fn main() {
    let mut console = Console::init();

    let minifont = font::FontHandle::init(&mut console, 0, SpriteRamQuadrant::One);

    loop {
        console.await_vblank();
        console.flip_framebuffer();

        console.draw_box(0, 0, 127, 127, 0b101_00_000);
        console.draw_box(0, 100, 127, 27, 0b011_10_110);

        // TODO: rename that to gt_string?
        minifont.draw_string(&mut console, 0, 1, &string_to_indices!("Hello traveler, it hath been a time"));
    }
}