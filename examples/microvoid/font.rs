use crate::system::console::{Console, Sprite, SpriteRamQuadrant};

pub struct FontHandle {
    bank: u8,
}

impl FontHandle {
    pub fn init(console: &mut Console) -> FontHandle {
        let sprite_sheet = dgtf_macros::include_spritesheet!("examples/microvoid/assets/minifont-p.bmp", "examples/microvoid/assets/minifont-p.json");

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

        FontHandle {
            bank: 0,
        }
    }
}


