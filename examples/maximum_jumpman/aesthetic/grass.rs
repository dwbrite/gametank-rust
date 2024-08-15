// use gt_crust::boot::wait;
use gt_crust::system::console::{Console, SpriteRamQuadrant};
use gt_crust::system::position::{Dimensions, FancyPosition, ScreenSpacePosition};
use gt_crust::system::sprite::{Sprite, VramBank};

pub struct Grass {
    pub array: [usize; 5], // 32*4 = 128, +1 for overscan
}

impl Grass {
    #[inline(never)]
    fn grass_draw_helper(&self, grass_idx: usize, position: &mut FancyPosition, console: &mut Console) {
        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[grass_idx + 6];
        let sprite = Sprite {
            bank: VramBank {
                bank: 0,
                quadrant: SpriteRamQuadrant::Two,
            },
            vram_x: sprite_data.sheet_x, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
            is_tile: false,
        };

        position.y = 101+64 - sprite.height; // draw right above floor (100)

        let mut dimensions = Dimensions {
            width: sprite.width,
            height: sprite.height,
        };

        let mut vram_position = ScreenSpacePosition {
            x: sprite.vram_x + sprite.bank.quadrant.value_gx(),
            y: sprite.vram_y + sprite.bank.quadrant.value_gy(),
        };

        let mut fb_position = position.to_screenspace();

        // don't render off screen
        if position.x >= 192 || position.y >= 192 ||
            position.x + dimensions.width <= 64  || position.y + dimensions.height <= 64 {
            return
        }

        if position.x < 64 {
            let off_screen_amount = 64 - position.x;
            dimensions.width = (dimensions.width + position.x) - 64;
            vram_position.x = vram_position.x + off_screen_amount; // add the change in width
            fb_position.x = 0;
        } else if dimensions.width > 192 - position.x {
            dimensions.width = 192 - position.x;
        }

        if position.y < 64 {
            let off_screen_amount = 64 - position.y;
            dimensions.height = (dimensions.height + position.y) - 64;
            vram_position.y = vram_position.y + off_screen_amount; // add the change in width
            fb_position.y = 0;
        } else if dimensions.height > 192 - position.y {
            dimensions.height = 192 - position.y;
        }

        // while console.blitter_registers.start.read() == 1 {}

        console.control_registers.set_dma_enable(true);

        console.control_registers.set_colorfill_mode(false);
        console.control_registers.set_vram_bank(sprite.bank.bank);
        console.control_registers.set_dma_gcarry(true);

        console.blitter_registers.vram_x.write(vram_position.x);
        console.blitter_registers.vram_y.write(vram_position.y);
        console.blitter_registers.fb_x.write(fb_position.x);
        console.blitter_registers.fb_y.write(fb_position.y);
        console.blitter_registers.width.write(dimensions.width);
        console.blitter_registers.height.write(dimensions.height - 1);
        console.blitter_registers.start.write(1);
    }

    #[inline(never)]
    pub fn draw_grass(&self, og_position: FancyPosition, console: &mut Console) {
        let mut position = FancyPosition {
            x: 0,
            y: 0,
        };
        for (c, &i) in self.array.iter().enumerate() {
            position.x = ((c * 32) - (og_position.x as usize)) as u8;
            self.grass_draw_helper(i, &mut position, console);
        }

        for i in 0..4 {
            position.x = (((5+i)*32) - (og_position.x as usize)) as u8;
            self.grass_draw_helper(self.array[i], &mut position, console);
        }
        // TODO: decide if this blitter wait is necessary
        // unsafe { wait(); }
        console.blitter_registers.reset_irq();
    }
}
