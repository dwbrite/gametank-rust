use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::background::{draw_background, draw_clouds};
use crate::aesthetic::grass::Grass;
use crate::font::FontHandle;
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::playing::Playing;
use crate::gamestates::start_menu::{STARTING_GRASS, StartMenu};
use crate::system::inputs::Buttons;

use crate::gamer::*;
use crate::system::position::FancyPosition;

pub struct Runup {
    pub minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    pub position: FancyPosition, // range of -128..255 as a "3 pane" recycled view
    pub(crate) grass: Grass,
    pub gamer: Gamer,
}

impl Runup {
    pub(crate) fn init(start_menu: StartMenu, mut console: &mut Console) -> Runup {
        Self {
            minifont: start_menu.minifont,
            position: start_menu.position,
            grass: Grass {
                array: start_menu.grass.array,
            },
            gamer: Gamer::init(console, 1, SpriteRamQuadrant::One),
        }
    }
}


impl GameState for Runup {
    fn update_and_draw(mut self, ticks: u64, mut console: &mut Console) -> GameStates {
        draw_background(console);
        draw_clouds(&self.position, console);
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
            if console.gamepad_1.is_pressed(Buttons::A) {
                return GameStates::Playing(Playing::init(self))
            }
        }

        self.position.x += 1;

        if self.position.x >= 255 {
            self.position.x = 0;
        }

        GameStates::Runup(self)
    }
}