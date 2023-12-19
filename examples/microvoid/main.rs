#![no_std]
#![no_main]

extern crate gt_crust;

pub mod system;

use core::{iter, mem};
use crate::system::scr::*;
use crate::system::bcr::*;
use crate::system::console::*;

#[no_mangle]
fn main() {
    let mut console = Console::init();

    let bmp_data = include_bytes!("assets/linminifont.bmp");

    #[repr(C)]
    struct BmpHeader {
        header: u16,
        size: u32,
        r0: u16,
        r1: u16,
        offset: u32
    }

    let header_iter = bmp_data.iter().take(14);
    let mut raw_header = [0u8; 14];
    for (i, byte) in header_iter.enumerate() {
        raw_header[i] = *byte; // Dereference to copy the valuez
    }

    let mut offset = 0;

    unsafe {
        let my_struct: BmpHeader = mem::transmute(raw_header);
        offset = my_struct.offset;
    }


    let width = 55; // Example width
    let height = 46; // Example height

    {
        let mut vram = console.access_vram_bank(0, SpriteRamQuadrant::One);

        for i in 0..128*height {
            vram.memory[i].write(0xff); // clear 20 lines of the vram buffer with white
        }
        let width_bytes = (width + 1) / 2;

        // Calculate row length with padding
        let row_length_with_padding = width_bytes + ((4 - (width_bytes % 4)) % 4);

        for pixel_y in 0..height {
            for pixel_x in (0..width).step_by(2) {
                // add offset, then add x/2, then add row_length*y
                let raw_idx = offset as usize + (pixel_x / 2) + (row_length_with_padding * pixel_y);
                let byte = bmp_data[raw_idx];

                let high_nibble: u8 = byte >> 4;
                let low_nibble: u8 = byte & 0x0F;

                vram.memory[pixel_x + pixel_y*128].write(high_nibble);
                vram.memory[pixel_x+1+pixel_y*128].write(low_nibble);
            }
        }
    }

    loop {
        console.await_vblank();

        console.flip_framebuffer();

        console.draw_box(0, 0, 127, 127, 0b010_00_100);
        //
        console.draw_box(0, 100, 127, 27, 0b111_10_101);

        let sprite = Sprite {
            bank: 0,
            vram_x: 0,
            vram_y: 0,
            width: width as u8,
            height: height as u8,
        };

        console.draw_sprite(&sprite, 0, 0, BlitMode::FlipY);
    }
}