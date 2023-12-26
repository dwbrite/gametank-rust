use crate::system::{console, sprite};
use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};
use crate::system::position::{FancyPosition, ScreenSpacePosition};
use crate::system::sprite::VramBank;
// creates a Sprite and SpriteSheet struct in this module, as well as a static SpriteSheet GAMER_SPRITES
dgtf_macros::include_spritesheet!(GAMER_SPRITES, "examples/microvoid/assets/gamer_con_polvo.bmp", "examples/microvoid/assets/gamer_con_polvo.json");

pub const STANDING: usize = 0;
pub const RUNNING: usize = 1;
pub const JUMPING: usize = 9;
pub const FALLING: usize = 10;
pub const SLIDING: usize = 11;
pub const FRAME_TIMES: [u8; 12] =   [0,  5,  5,  5,  5,  5,  5,  5,  4,  0,  0,  0]; // this was 3x what it should be at 60fps???
pub const X_OFFSET: [u8; 12] =      [2,  0,  1,  2,  2,  0,  2,  2,  2,  2,  2,  6];
pub const Y_OFFSET: [u8; 12] =      [0,  0,  0,  0,  1,  0,  0,  0,  0,  0,  0,  0];



pub enum GamerStates {
    Standing,
    Running,
    Jumping,
    Falling,
    Sliding,
    // TODO: dying? lol
}
pub struct Gamer {
    pub bank: u8,
    pub quadrant: SpriteRamQuadrant,
    pub spritesheet: &'static SpriteSheet,
    pub frame_counter: u8,
    pub animation: usize,
    pub state: GamerStates,
    pub y: u8,

}

impl Gamer {
    pub fn init(console: &mut Console, bank: u8, quadrant: SpriteRamQuadrant) -> Self {
        let sprite_sheet = &GAMER_SPRITES;
        let mut vram = console.access_vram_bank(bank, &quadrant);

        let bits_per_pixel = 8 / sprite_sheet.pixels_per_byte as usize;
        let mask = (1 << bits_per_pixel) - 1;

        // TODO: probably extract this lol
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
                let input = x + y * sprite_sheet.width as usize;
                let output = x + (y + 40) * 128;

                vram.memory[output].write(vram.memory[input].read());
            }
        }

        Self {
            bank,
            quadrant,
            spritesheet: sprite_sheet,
            frame_counter: 0,
            animation: FALLING,
            state: GamerStates::Falling,
            y: 100 - sprite_sheet.sprite_data[FALLING].height,
        }
    }

    pub fn set_animation(&mut self, anim: usize) {
        self.animation = anim
    }

    pub fn update_and_draw(&mut self, mut console: &mut Console) {
        let sprite_data = self.spritesheet.sprite_data[self.animation];
        let sprite = sprite::Sprite {
            bank: VramBank {
                bank: self.bank,
                quadrant: self.quadrant.clone(),
            },
            vram_x: sprite_data.sheet_x, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
        };

        let position = ScreenSpacePosition {
            x: 24 - X_OFFSET[self.animation],
            y: 100 - sprite.height - self.y - Y_OFFSET[self.animation],
        };

        // TODO, y calculation is jank/temporary. We'll need hitboxes and sprite offsets.
        sprite.draw_sprite(position, BlitMode::Normal, console);

        self.frame_counter += 1;

        // TODO: this could _probably_ be improved, right?
        if self.frame_counter > FRAME_TIMES[self.animation] {
            self.frame_counter = 0;
            match self.animation {
                1..=7 => self.animation += 1,
                8 => self.animation = 1,
                _ => {}
            }
        }
    }
}