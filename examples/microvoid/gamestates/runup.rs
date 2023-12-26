use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::grass::Grass;
use crate::font::FontHandle;
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::playing::Playing;
use crate::gamestates::start_menu::{STARTING_GRASS, StartMenu};
use crate::system::inputs::Buttons;

use crate::gamer::*;

pub struct Runup {
    pub minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    pub position: i16, // range of -128..255 as a "3 pane" recycled view
    pub(crate) grass: Grass,
    pub gamer: Gamer,
}

impl Runup {
    pub(crate) fn init(start_menu: StartMenu, mut console: &mut Console) -> Runup {
        Self {
            minifont: start_menu.minifont,
            position: start_menu.position,
            grass: start_menu.grass,
            gamer: Gamer::init(console, 1, SpriteRamQuadrant::One),
        }
    }

    pub(crate) fn draw_background(&self, console: &mut Console) {
        console.draw_box(0, 0, 127, 100, 0b101_00_000);
        console.draw_box(0, 100, 127, 28, 0b011_10_110);
        console.draw_box(127, 100, 1, 28, 0b011_10_110);
        console.draw_box(127, 0, 1, 100, 0b101_00_000);
    }

    fn draw_clouds(&mut self, mut console: &mut Console) {
        let cloud_1 = (128, 9);
        let cloud_2 = (256+32, 30);

        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[3];
        let sprite = crate::system::console::Sprite {
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


impl GameState for Runup {
    fn update_and_draw(mut self, ticks: u64, mut console: &mut Console) -> GameStates {
        self.draw_background(console);
        self.draw_clouds(console);
        // TODO: debug mystery rendering glitch, even though this is exactly the same as start_menu.
        // fml
        self.grass.draw_grass(self.position, console);

        self.gamer.update_and_draw(console);

        let is_running = self.gamer.animation >= RUNNING && self.gamer.animation < JUMPING;

        if self.gamer.y > 0 {
            self.gamer.y -= 1;
        } else if self.gamer.animation == FALLING {
            self.gamer.animation = RUNNING
        } else if is_running {
            self.minifont.draw_string(console, 64, 72, &string_to_indices!("hold A to jump"));
            if console.gamepad_1.is_pressed(&Buttons::A) {
                return GameStates::Playing(Playing::init(self))
            }
        }

        self.position += 1;

        if self.position >= (128*3) {
            self.position = 0;
        }

        GameStates::Runup(self)
    }
}