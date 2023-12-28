use fixed::FixedI16;
use fixed::types::extra::U8;
use crate::system::console::{Console, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::background::{draw_background, draw_clouds};
use crate::aesthetic::grass::Grass;
use crate::font::FontHandle;
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::playing::Playing;
use crate::gamestates::start_menu::{StartMenu};
use crate::system::inputs::Buttons;

use crate::gamer::*;
use crate::system::position::FancyPosition;

pub struct Runup {
    pub minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    pub position: FancyPosition, // range of -128..255 as a "3 pane" recycled view
    pub grass: Grass,
    pub gamer: Gamer,
    pub timer: u16,
}

impl Runup {
    pub(crate) fn init(start_menu: StartMenu, console: &mut Console) -> Runup {
        Self {
            minifont: start_menu.minifont,
            position: start_menu.position,
            grass: Grass {
                array: start_menu.grass.array,
            },
            gamer: Gamer::init(console, 1, SpriteRamQuadrant::One),
            timer: 0,
        }
    }
}


impl GameState for Runup {
    fn update_and_draw(mut self, _ticks: u64, console: &mut Console) -> GameStates {
        let is_running = self.gamer.state == GamerStates::Running;

        draw_background(console, false);
        draw_clouds(&self.position, console);
        self.grass.draw_grass(self.position, console);
        self.gamer.update_and_draw(console);

        if !is_running {
            self.gamer.holding_jump = true;
            // temporarily reduce gravity, for aesthetic reasons
            self.gamer.velocity -= FixedI16::<U8>::from_num(0.015);
        } else if self.timer >= 1 { // wait 3 seconds TODO set this back to 180
            self.minifont.draw_string(console, 64, 72, &string_to_indices!("hold A to jump"));
            if console.gamepad_1.is_pressed(Buttons::A) {
                return GameStates::Playing(Playing::init(self))
            }
        }

        if self.timer < 20000 {
            self.timer += 1;
        }

        // allow overflow
        self.position.x += 1;

        GameStates::Runup(self)
    }
}