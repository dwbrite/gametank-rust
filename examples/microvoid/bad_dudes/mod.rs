use fixed::FixedI16;
use fixed::traits::LossyInto;

use gt_crust::system::console::{BlitMode, Console, SpriteRamQuadrant};
use gt_crust::system::position::{f16u8_to_u8, FancyPosition, SubpixelFancyPosition};
use gt_crust::system::rectangle::Rectangle;
use gt_crust::system::sprite::{Sprite, VramBank};

#[enum_delegate::implement(BadDude)]
pub enum BadDudes {
    NoDude(NoDude),
    LineDude(LineDude),
}

#[enum_delegate::register]
pub trait BadDude {
    fn update_and_draw(&mut self, velocity_x: FixedI16<8>, console: &mut Console);
    fn hitbox(&self) -> Rectangle;

    fn is_offscreen_left(&self) -> bool;
}

pub struct NoDude {}

impl BadDude for NoDude {
    fn update_and_draw(&mut self, velocity_x: FixedI16<8>, _console: &mut Console) {
        // do nothing
    }

    fn hitbox(&self) -> Rectangle {
        Rectangle {
            xy: FancyPosition { x: 0, y: 0 },
            size: FancyPosition { x: 0, y: 0 },
        }
    }

    fn is_offscreen_left(&self) -> bool {
        true
    }
}

pub struct LineDude {
    pub position: SubpixelFancyPosition,
    pub height: u8, // must be %2
}

impl BadDude for LineDude {
    fn update_and_draw(&mut self, velocity_x: FixedI16<8>, console: &mut Console) {
        self.position.x = self.position.x.sub_signed(velocity_x);

        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[0];
        let mut sprite = Sprite {
            bank: VramBank {
                bank: 0,
                quadrant: SpriteRamQuadrant::Two,
            },
            vram_x: sprite_data.sheet_x,
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: if self.height < 18 { self.height } else { 18 } ,
            is_tile: false,
        };

        // draw top first
        let mut draw_position = self.position.to_fancy();
        draw_position.y -= self.height;

        sprite.draw_sprite_with_overscan(draw_position, BlitMode::Normal, console);

        let mut working_height = self.height - sprite.height;

        if working_height == 0 {
            return
        }

        // move onto mid piece
        sprite.height -= 2;
        sprite.vram_y += 2;

        while working_height > 0 {
            if working_height <= 16 {
                sprite.height = working_height;
            }

            draw_position.y = self.position.to_fancy().y - working_height;
            sprite.draw_sprite_with_overscan(draw_position, BlitMode::Normal, console);
            working_height -= sprite.height;
        }
    }

    fn hitbox(&self) -> Rectangle {
        let mut xy = self.position.to_fancy();
        xy.y -= self.height;

        Rectangle {
            xy,
            size: FancyPosition { x: 9, y: self.height },
        }
    }

    fn is_offscreen_left(&self) -> bool {
        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[0];

        self.position.to_fancy().x + sprite_data.width <= 64
    }
}