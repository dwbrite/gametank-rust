#![no_std]
#![no_main]

extern crate gt_crust;

pub mod system;

use crate::system::scr::*;
use crate::system::bcr::*;
use crate::system::console::*;

#[no_mangle]
fn main() {
    let mut console = Console::init();

    let bmp_data = include_bytes!("assets/linminifont.bmp");
    let bmp_width = 55;
    let bmp_height = 46;


    let mut vram = console.access_vram_bank(0, SpriteRamQuadrant::One);

    for i in 0..128*bmp_height {
        vram.memory[i ].write(0xff); // clear 20 lines of the vram buffer with white
    }

    for (i, byte) in bmp_data.iter().skip(117).enumerate() {
        let high_nibble = byte >> 4; // Extracts the high nibble
        let low_nibble = byte & 0x0F; // Extracts the low nibble

        let target_idx = i * 2;
        let x = target_idx % bmp_width;
        let y = target_idx / bmp_width;

        let pair_final_idx = x + y * 128;

        vram.memory[pair_final_idx].write(low_nibble);
        vram.memory[pair_final_idx+1].write(high_nibble);
    }

    loop {
        console.await_vblank();

        console.flip_framebuffer();

        console.draw_box(0, 0, 127, 127, 0b010_00_100);
        //
        console.draw_box(0, 100, 127, 27, 0b111_10_101);

        console.draw_sprite(&Sprite {
            bank: 0,
            vram_x: 0,
            vram_y: 0,
            width: bmp_width as u8,
            height: bmp_height as u8,
        }, 2, 2);
    }
}