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

    let bmp_data = include_bytes!("assets/minifont.bmp");


    let mut vram = console.access_vram_bank(0, SpriteRamQuadrant::One);

    for i in 0..128*20 {
        vram.memory[i ].write(0xff); // clear 20 lines of the vram buffer with white
    }

    loop {
        for (i, byte) in bmp_data.iter().skip(118).enumerate() {
            let high_nibble = byte >> 4; // Extracts the high nibble
            let low_nibble = byte & 0x0F; // Extracts the low nibble

            let target_idx = i * 2;
            let x = target_idx % 30;
            let y = target_idx / 30;
            let pair_final_idx = x + (y << 7);

            vram.memory[pair_final_idx].write(high_nibble);
            vram.memory[pair_final_idx+1].write(low_nibble);
        }
    }
        // let pn = i*2;
        // let x = pn % 30;
        // let y = p   n / 30;
        //
        // if y > 5 {
        //     vram.memory[i*2].write(123);
        //     vram.memory[1+i*2].write(123);
        // }
        //
        // vram.memory[x +  y*128 + 10*128].write(high_nibble);
        // vram.memory[x+1+ y*128 + 10*128].write(low_nibble);
    //
    // let mut vram = console.access_vram_bank(0, SpriteRamQuadrant::One);
    //
    // for i in 0..15*30 {
    //
    //     let pn = i*2;
    //     let x = pn % 30;
    //     let y = pn / 30;
    //
    //     if y > 5 {
    //         vram.memory[i*2 + 40*128].write(123);
    //         vram.memory[1+i*2 + 40*128].write(123);
    //     }
    //
    //     vram.memory[x +  y*128 + 60*128].write(123);
    //     vram.memory[x+1+ y*128 + 60*128].write(123);
    // }

    //
    // for i in 0..len {
    //     let x = i % 30;
    //     let y = i / 30;
    //     vram.memory[x + y*128] = vram.memory[i];
    // }

    // loop {
        // console.await_vblank();
        //
        // console.flip_framebuffer();
        //
        // console.draw_box(0, 0, 127, 127, 0b010_00_100);
        // //
        // console.draw_box(0, 100, 127, 27, 0b111_10_101);

        // console.draw_sprite(&s, 2, 2);
    // }
}