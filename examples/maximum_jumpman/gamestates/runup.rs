use fixed::{FixedI16, FixedU16};

use gt_crust::system::console::{Console, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::background::draw_background;
use crate::aesthetic::grass::Grass;
use crate::font::FontHandle;
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::playing::Playing;
use gt_crust::system::inputs::Buttons;

use crate::gamer::*;
use gt_crust::system::position::SubpixelFancyPosition;

// TODO: preload rng tiles which are possible

pub struct Runup {
    pub minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    pub position: SubpixelFancyPosition, // range of -128..255 as a "3 pane" recycled view
    pub grass: Grass,
    pub gamer: Gamer,
    pub timer: u16,
    pub velocity: FixedI16<8>,
}

impl Runup {
    pub(crate) fn init(console: &mut Console) -> Runup {
        let minifont = FontHandle::init(console, 0, gt_crust::system::console::SpriteRamQuadrant::One);

        Self {
            minifont,
            position: SubpixelFancyPosition {
                x: FixedU16::<8>::from_num(0),
                y: FixedU16::<8>::from_num(0),
            },
            grass:  Grass {
                array: STARTING_GRASS,
            },
            gamer: Gamer::init(console, 1, SpriteRamQuadrant::One),
            timer: 0,
            velocity: FixedI16::<8>::from_num(1.3),
        }
    }
}


impl GameState for Runup {
    fn update_and_draw(mut self, _ticks: u64, console: &mut Console) -> GameStates {
        let is_running = self.gamer.state == GamerStates::Running;

        draw_background(console, true); // this brings us down to 30fps lol
        // draw_clouds(&self.position.to_fancy(), console);
        self.grass.draw_grass(self.position.to_fancy(), console);
        self.minifont.draw_string(console, 28, 40, &string_to_indices!("Maximum Jumpman"));
        self.minifont.draw_string(console, 28, 46, &string_to_indices!("Chronicles"));
        self.gamer.update_and_draw(self.velocity, console);

        if !is_running {
            self.gamer.holding_jump = true;
            // temporarily reduce gravity, for aesthetic reasons
            self.gamer.velocity -= FixedI16::<8>::from_num(0.015);
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
        self.position.x = self.position.x.add_signed(self.velocity);

        GameStates::Runup(self)
    }
}


pub const STARTING_GRASS: [usize; 5] = [0, 5, 6, 3, 4];
