use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};
use crate::system::position::{Dimensions, FancyPosition, ScreenSpacePosition};

pub struct VramBank {
    pub bank: u8,
    pub quadrant: SpriteRamQuadrant
}

pub struct Sprite {
    pub bank: VramBank,
    pub vram_x: u8,
    pub vram_y: u8,
    pub width: u8,
    pub height: u8,
    pub is_tile: bool,
    pub with_interrupt: bool,
}


impl SpriteRamQuadrant {
    pub fn value_gx(&self) -> u8 {
        match self {
            Self::One | Self::Three => 0,
            Self::Two | Self::Four => 128
        }
    }

    pub fn value_gy(&self) -> u8 {
        match self {
            Self::One | Self::Two => 0,
            Self::Three | Self::Four => 128
        }
    }
}

impl Sprite {

    #[inline(always)]
    pub fn send_blit(&self, fb_position: &ScreenSpacePosition, vram_position: &ScreenSpacePosition, dimensions: &Dimensions, console: &mut Console) {
        console.control_registers.set_dma_enable(true);

        console.control_registers.set_colorfill_mode(false);
        console.control_registers.set_vram_bank(self.bank.bank);
        console.control_registers.set_dma_gcarry(!self.is_tile);

        console.blitter_registers.vram_x.write(vram_position.x);
        console.blitter_registers.vram_y.write(vram_position.y);
        console.blitter_registers.fb_x.write(fb_position.x);
        console.blitter_registers.fb_y.write(fb_position.y);
        console.blitter_registers.width.write(dimensions.width);
        console.blitter_registers.height.write(dimensions.height);
        console.blitter_registers.start.write(1);


        if self.with_interrupt {
            unsafe { gt_crust::boot::wait(); }
            console.blitter_registers.reset_irq();
        } else {
            while console.blitter_registers.start.read() == 1 {}
        }
    }

    // TODO: add function to draw sprite without waiting, for optimization - maybe add private "inline always" version for code-reuse
    pub fn draw_sprite(&self, fb_position: ScreenSpacePosition, blit_mode: BlitMode, console: &mut Console) {
        let mut dimensions = Dimensions {
            width: self.width,
            height: self.height,
        };

        let mut vram_position = ScreenSpacePosition {
            x: self.vram_x + self.bank.quadrant.value_gx(),
            y: self.vram_y + self.bank.quadrant.value_gy(),
        };

        if blit_mode == BlitMode::FlipX || blit_mode == BlitMode::FlipXY {
            vram_position.x ^= 0xFF;
            vram_position.x -= dimensions.width - 1;
            dimensions.width ^= 0b10000000;
        }

        if blit_mode == BlitMode::FlipY || blit_mode == BlitMode::FlipXY {
            vram_position.y ^= 0xFF;
            vram_position.y -= dimensions.height - 1;
            dimensions.height ^= 0b10000000;
        }

        self.send_blit(&fb_position, &vram_position, &dimensions, console)
    }



    pub fn draw_sprite_with_overscan(&self, position: FancyPosition, blit_mode: BlitMode, console: &mut Console) {
        let mut dimensions = Dimensions {
            width: self.width,
            height: self.height,
        };

        let mut vram_position = ScreenSpacePosition {
            x: self.vram_x + self.bank.quadrant.value_gx(),
            y: self.vram_y + self.bank.quadrant.value_gy(),
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

        if blit_mode == BlitMode::FlipX || blit_mode == BlitMode::FlipXY {
            vram_position.x ^= 0xFF;
            vram_position.x -= dimensions.width - 1;
            dimensions.width ^= 0b10000000;
        }

        if blit_mode == BlitMode::FlipY || blit_mode == BlitMode::FlipXY {
            vram_position.y ^= 0xFF;
            vram_position.y -= dimensions.height - 1;
            dimensions.height ^= 0b10000000;
        }

        // TODO: maybe we can curry send_blit?
        self.send_blit(&fb_position, &vram_position, &dimensions, console)
    }
}