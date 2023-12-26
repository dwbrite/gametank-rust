use crate::font::FontHandle;
use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::background::{draw_background, draw_clouds};
use crate::aesthetic::grass::Grass;
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::runup::Runup;
use crate::system::inputs::Buttons;
use crate::system::position::FancyPosition;
use crate::system::sprite::{Sprite, VramBank};

pub const STARTING_GRASS: [usize; 5] = [0, 5, 6, 3, 4];

pub struct StartMenu {
    pub minifont: FontHandle,
    pub position: FancyPosition,
    pub grass: Grass,
    is_seeded: bool,
}

impl StartMenu {
    pub(crate) fn init(mut console: &mut Console) -> StartMenu {
        let minifont = FontHandle::init(&mut console, 0, crate::system::console::SpriteRamQuadrant::One);

        Self {
            minifont,
            position: FancyPosition {
                x: 0, y: 0
            },
            grass: Grass {
                array: STARTING_GRASS,
            },
            is_seeded: false,
        }
    }


    fn draw_start_text(&mut self, ticks: u64, mut console: &mut Console) {
        let y_offset = (ticks % (78)) / 26; // 3 states, 26 ticks long each
        self.minifont.draw_string(&mut console, 30, 80 - (y_offset as u8), &string_to_indices!("Press Start, Gamer"));
    }

}


impl GameState for StartMenu {
    fn update_and_draw(mut self, ticks: u64, mut console: &mut Console) -> GameStates {
        draw_background(console);
        draw_clouds(&self.position, console);
        self.draw_start_text(ticks, console);
        self.grass.draw_grass(self.position, console);


        if self.is_seeded && ticks % 16 == 0 {
            return GameStates::Runup(Runup::init(self, &mut console))
        }

        if console.gamepad_1.is_pressed(Buttons::Start) {
            console.preseed_rng(ticks);
            self.is_seeded = true;
        }

        self.position.x += 1;

        if self.position.x >= (255) {
            self.position.x = 0;
        }

        GameStates::StartMenu(self)
    }
}