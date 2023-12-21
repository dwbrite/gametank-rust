#![no_std]
#![no_main]

extern crate gt_crust;

pub mod system;
use core::{iter, mem};
use core::iter::once;
use crate::system::scr::*;
use crate::system::bcr::*;
use crate::system::console::*;

#[no_mangle]
fn main() {
    let mut console = Console::init();

    let sprite_sheet = dgtf_macros::include_spritesheet!("examples/microvoid/assets/minifont-sp.bmp");

    let mut vram = console.access_vram_bank(0, SpriteRamQuadrant::One);

    let bits_per_pixel = 8 / sprite_sheet.pixels_per_byte as usize;
    let mask = (1 << bits_per_pixel) - 1;

    for byte_index in 0..sprite_sheet.pixel_array.len() {
        let byte = sprite_sheet.pixel_array[byte_index];

        for idx_idx in 0..(sprite_sheet.pixels_per_byte as usize) {
            let pixel_index = (byte >> (bits_per_pixel * idx_idx)) & mask;
            let color = sprite_sheet.palette[pixel_index as usize];

            let overall_pixel_index = byte_index * sprite_sheet.pixels_per_byte as usize + idx_idx;

            vram.memory[overall_pixel_index].write(color);
        }
    }

    for y in 0..sprite_sheet.height as usize {
        for x in 0..sprite_sheet.width as usize {
            let input = x+y*sprite_sheet.width as usize;
            let output = x + (y+40)*128;

            vram.memory[output].write(vram.memory[input].read());
        }
    }

    loop {
        console.await_vblank();
        console.flip_framebuffer();

        console.draw_box(0, 0, 127, 127, 0b010_00_100);
        console.draw_box(0, 100, 127, 27, 0b111_10_101);

        let sprite = Sprite {
            bank: 0,
            vram_x: 0,
            vram_y: 40,
            width: sprite_sheet.width,
            height: sprite_sheet.height,
        };

        console.draw_sprite(&sprite, 0, 0, BlitMode::Normal);
    }
}