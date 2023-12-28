use fixed::FixedI16;
use fixed::types::extra::U8;
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
    pub grass: Grass,
    pub gamer: Gamer,
    pub timer: u16,
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
            timer: 0,
        }
    }
}


impl GameState for Runup {
    fn update_and_draw(mut self, ticks: u64, mut console: &mut Console) -> GameStates {
        let is_running = self.gamer.state == GamerStates::Running;

        draw_background(console);
        draw_clouds(&self.position, console);
        self.grass.draw_grass(self.position, console);
        self.gamer.update_and_draw(console);

        if !is_running {
            self.gamer.holding_jump = true;
            // temporarily reduce gravity, for aesthetic reasons
            self.gamer.velocity -= FixedI16::<U8>::from_num(0.015);
        } else if self.timer >= 180 { // wait 2 seconds
            self.minifont.draw_string(console, 64, 72, &string_to_indices!("hold A to jump"));
            if console.gamepad_1.is_pressed(Buttons::A) {
                return GameStates::Playing(Playing::init(self))
            }
        }

        if self.timer < 20000 {
            self.timer += 1;
        }

        self.position.x += 1;

        if self.position.x >= 255 {
            self.position.x = 0;
        }

        GameStates::Runup(self)
    }
}