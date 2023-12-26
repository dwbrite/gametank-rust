use gt_crust::boot::wait;
use crate::system::console::{Console, Sprite};

pub struct Grass {
    pub array: [usize; 12],
}

impl Grass {
    #[inline(always)]
    fn grass_draw_helper(&self, grass_idx: usize, x: i16, is_first: bool, console: &mut Console) {
        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[grass_idx + 8];
        let sprite = Sprite {
            bank: 0,
            vram_x: sprite_data.sheet_x + 128, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
        };

        let y = (101 - sprite.height) as i16;

        let (mut width, mut height) = (sprite.width, sprite.height);
        let (mut vram_x, mut vram_y) = (sprite.vram_x, sprite.vram_y);

        let (mut new_x, mut new_y) = (x as u8, y as u8);

        // don't render if off screen
        if x > 127 || x <= -(width as i16) {
            return
        }


        if x < 0 {
            width = (width as i16 + x) as u8;
            vram_x = (vram_x as i16 - x) as u8;
            new_x = 0;
        } else if width > (128 - x) as u8 {
            width = (128 - x) as u8;
        }


        if !is_first {
            while console.blitter_registers.start.read() == 1 {}
        } else {
            console.control_registers.set_dma_enable(true);

            console.control_registers.set_colorfill_mode(false);
            console.control_registers.set_vram_bank(sprite.bank);
            console.control_registers.set_dma_gcarry(true);
        }

        console.blitter_registers.vram_x.write(vram_x);
        console.blitter_registers.vram_y.write(vram_y);
        console.blitter_registers.fb_x.write(new_x);
        console.blitter_registers.fb_y.write(new_y);
        console.blitter_registers.width.write(width);
        console.blitter_registers.height.write(height-1);
        console.blitter_registers.start.write(1);
    }

    #[inline(always)]
    pub fn draw_grass(&self, position: i16, console: &mut Console) {
        for (c, i) in self.array.iter().enumerate() {
            self.grass_draw_helper(*i, (c*32) as i16 - (position), c==0, console);
        }
        // redraw the first 4 at the end to loop smoothly
        for c in 0..4 {
            self.grass_draw_helper(self.array[c], ((c+12)*32) as i16 - (position), c==0, console);
        }
        // TODO: decide if this blitter wait is necessary
        unsafe { wait(); }
    }
}
