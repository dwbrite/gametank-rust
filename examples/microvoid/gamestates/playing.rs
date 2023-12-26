use crate::font::FontHandle;
use crate::system::console::{BlitMode, Console, Sprite, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::grass::{Grass};
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::runup::Runup;
use crate::gamestates::start_menu::StartMenu;
use crate::system::inputs::Buttons;
use crate::gamer::*;

pub struct Playing {
    minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    position: i16, // range of -128..255 as a "3 pane" recycled view
    gamer: Gamer,
    grass: Grass,
    score: u64,
}

impl Playing {
    pub(crate) fn init(runup: Runup) -> Playing{
        Self {
            minifont: runup.minifont,
            position: runup.position,
            score: 0,
            grass: runup.grass,
            gamer: runup.gamer,
        }
    }

    pub(crate) fn draw_background(&self, console: &mut Console) {
        console.draw_box(0, 0, 127, 100, 0b101_00_000);
        console.draw_box(0, 100, 127, 28, 0b011_10_110);
        console.draw_box(127, 100, 1, 28, 0b011_10_110);
        console.draw_box(127, 0, 1, 100, 0b101_00_000);
        // TODO: replace the big draw with one that doesn't wait? would be a good time to gen rng
        // or to calculate the next section? or calculate hitboxes?
    }

    fn draw_clouds(&mut self, mut console: &mut Console) {
        let cloud_1 = (128, 9);
        let cloud_2 = (256+32, 30);

        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[3];
        let sprite = Sprite {
            bank: 0,
            vram_x: sprite_data.sheet_x + 128, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
        };

        sprite.draw_sprite_with_overscan(cloud_1.0 - self.position, cloud_1.1, BlitMode::Normal, &mut console);
        sprite.draw_sprite_with_overscan(cloud_2.0 - self.position, cloud_2.1, BlitMode::Normal, &mut console);
    }
}


impl GameState for Playing {
    fn update_and_draw(mut self, ticks: u64, mut console: &mut Console) -> GameStates {
        self.draw_background(console);
        // self.draw_clouds(console);
        self.grass.draw_grass(self.position, console);

        self.gamer.update_and_draw(console);

        if ticks % 16 == 0 {
            self.score += 1;
        }

        self.position += 1;

        if self.position >= (128*3) {
            self.position = 0;
        }

        GameStates::Playing(self)
    }
}