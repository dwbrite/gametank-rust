use crate::font::FontHandle;
use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::background::{draw_background, draw_clouds};
use crate::aesthetic::grass::{Grass};
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::runup::Runup;
use crate::gamestates::start_menu::StartMenu;
use crate::system::inputs::Buttons;
use crate::gamer::*;
use crate::system::position::FancyPosition;
use crate::system::sprite::VramBank;

pub struct Playing {
    minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    position: FancyPosition, // range of -64..192 as a 0.5+1.0+0.5 screen, recycled view
    gamer: Gamer,
    grass: Grass,
    score: u16,
    silly_digits: u8,
}

impl Playing {
    pub(crate) fn init(runup: Runup) -> Playing{
        Self {
            minifont: runup.minifont,
            position: runup.position,
            score: 36000,
            grass: runup.grass,
            gamer: runup.gamer,
            silly_digits: 0,
        }
    }
}


impl GameState for Playing {
    fn update_and_draw(mut self, ticks: u64, mut console: &mut Console) -> GameStates {
        draw_background(console);
        draw_clouds(&self.position, console);
        self.grass.draw_grass(self.position, console);
        self.gamer.update_and_draw(console);

        self.minifont.draw_number(console, 0, 0, self.score, self.silly_digits);

        self.silly_digits += 1;

        if self.silly_digits >= 100 {
            self.score += 8;
            self.silly_digits -= 100;
        }

        self.position.x += 1;

        if self.position.x >= (255) {
            self.position.x = 0;
        }

        GameStates::Playing(self)
    }
}