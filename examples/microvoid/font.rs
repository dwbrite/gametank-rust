use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};

// creates a Sprite and SpriteSheet struct in this module, as well as a static SpriteSheet MINIFONT_SPRITES
dgtf_macros::include_spritesheet!(MINIFONT_SPRITES, "examples/microvoid/assets/minifont-p.bmp", "examples/microvoid/assets/minifont-p.json");

pub struct FontHandle {
    bank: u8,
    quadrant: SpriteRamQuadrant,
    spritesheet: &'static SpriteSheet
}

impl FontHandle {
    pub fn init(console: &mut Console, bank: u8, quadrant: SpriteRamQuadrant) -> FontHandle {
        let sprite_sheet = &MINIFONT_SPRITES;

        let mut vram = console.access_vram_bank(bank, &quadrant);

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
            bank,
            quadrant,
            spritesheet: &MINIFONT_SPRITES
        }
    }

    pub fn draw_string(&self, console: &mut Console, x: u8, y: u8, string: &[usize]) {
        let mut w = 0;
        let mut is_first = true;
        for char in string {
            let c = crate::Sprite {
                bank: self.bank,
                vram_x: self.spritesheet.sprite_data[*char].sheet_x,
                vram_y: 40 + self.spritesheet.sprite_data[*char].sheet_y,
                width: self.spritesheet.sprite_data[*char].width,
                height: self.spritesheet.sprite_data[*char].height,
            };
            let x_offset = self.spritesheet.sprite_data[*char].x_offset;
            let y_offset = self.spritesheet.sprite_data[*char].y_offset;
            let x = w + x + x_offset;
            let y = y + y_offset;

            w += c.width + x_offset;

            if *char == 0 {
                w -= 1
            }

            string_draw_helper(c, x, y, is_first, console);
            is_first = false;
            //
            // c.draw_sprite(x, y, BlitMode::Normal, console);
        }
    }
}

fn string_draw_helper(sprite: crate::Sprite, x: u8, y: u8, is_first: bool, console: &mut Console) {
    if !is_first {
        while console.blitter_registers.start.read() == 1 {}
    } else {
        console.control_registers.set_dma_enable(true);

        console.control_registers.set_colorfill_mode(false);
        console.control_registers.set_vram_bank(sprite.bank);
        console.control_registers.set_dma_gcarry(true);
    }

    console.blitter_registers.vram_x.write(sprite.vram_x);
    console.blitter_registers.vram_y.write(sprite.vram_y);
    console.blitter_registers.fb_x.write(x);
    console.blitter_registers.fb_y.write(y);
    console.blitter_registers.width.write(sprite.width);
    console.blitter_registers.height.write(sprite.height);
    console.blitter_registers.start.write(1);
}
