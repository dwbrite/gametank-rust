use gt_crust::boot::wait;
use crate::system::console::{Console, SpriteRamQuadrant};
use crate::system::sprite;
use crate::system::sprite::VramBank;

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

        let vram = console.access_vram_bank(bank, &quadrant);

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
        for char in string {
            let c = sprite::Sprite {
                bank: VramBank {
                    bank: self.bank,
                    quadrant: self.quadrant,
                },
                vram_x: self.spritesheet.sprite_data[*char].sheet_x,
                vram_y: 40 + self.spritesheet.sprite_data[*char].sheet_y,
                width: self.spritesheet.sprite_data[*char].width,
                height: self.spritesheet.sprite_data[*char].height,
                is_tile: false,
            };
            let x_offset = self.spritesheet.sprite_data[*char].x_offset;
            let y_offset = self.spritesheet.sprite_data[*char].y_offset;

            let x = w + x + x_offset;
            let y = y + y_offset;

            w += c.width + x_offset;

            if *char == 0 {
                w -= 1
            }



            string_draw_helper(c, x, y, console);
        }
        // unsafe { wait() }
        console.blitter_registers.reset_irq();
    }

    pub fn digit_to_char(c: u8) -> usize {

        if c == 0 {
            62
        } else {
            (c + 52) as usize
        }
    }

    // max 65535_00
    pub fn draw_number(&self, console: &mut Console, x: u8, y: u8, number: u16, silly_digits: u8) {
        let mut remainder = number;
        let mut digits = [0; 7];
        let mut w = 0;

        let mut beyond_front = false;

        for (i, &n) in [10000, 1000, 100, 10, 1, 10, 1].iter().enumerate() {
            if i >= 5 {
                remainder = silly_digits as u16;
            }

            if n == 1 {
                digits[i] = remainder % 10;
            } else {
                digits[i] = remainder / n;
            }
            remainder -= digits[i] * n;

            let char = Self::digit_to_char(digits[i] as u8);

            let c = sprite::Sprite {
                bank: VramBank {
                    bank: self.bank,
                    quadrant: self.quadrant,
                },
                vram_x: self.spritesheet.sprite_data[char].sheet_x,
                vram_y: 40 + self.spritesheet.sprite_data[char].sheet_y,
                width: self.spritesheet.sprite_data[char].width,
                height: self.spritesheet.sprite_data[char].height,
                is_tile: false,
            };
            let x_offset = self.spritesheet.sprite_data[char].x_offset;
            let y_offset = self.spritesheet.sprite_data[char].y_offset;
            let x = w + x + x_offset;
            let y = y + y_offset;

            if char != 62 {
                beyond_front = true;
            } else if !beyond_front && i < 6 {
                // if char is '0'
                // and we haven't printed another character
                // and we're not at the end
                continue
            }

            w += c.width + x_offset;

            string_draw_helper(c, x, y, console);
        }
        unsafe { wait(); }
        console.blitter_registers.reset_irq();
    }
}

fn string_draw_helper(sprite: sprite::Sprite, x: u8, y: u8, console: &mut Console) {
    console.control_registers.set_dma_enable(true);

    console.control_registers.set_colorfill_mode(false);
    console.control_registers.set_vram_bank(sprite.bank.bank);
    console.control_registers.set_dma_gcarry(true);

    console.blitter_registers.vram_x.write(sprite.vram_x);
    console.blitter_registers.vram_y.write(sprite.vram_y);
    console.blitter_registers.fb_x.write(x);
    console.blitter_registers.fb_y.write(y);
    console.blitter_registers.width.write(sprite.width);
    console.blitter_registers.height.write(sprite.height);
    console.blitter_registers.start.write(1);
}
