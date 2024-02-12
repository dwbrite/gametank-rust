use fixed::{FixedI16, FixedU16};

use crate::font::FontHandle;
use gt_crust::system::console::{Console};
use dgtf_macros::string_to_indices;
use crate::aesthetic::background::{draw_background, draw_clouds};
use crate::aesthetic::grass::Grass;
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::runup::Runup;
use gt_crust::system::inputs::Buttons;
use gt_crust::system::position::{FancyPosition, SubpixelFancyPosition};
use gt_crust::system::console::SpriteRamQuadrant;

pub const STARTING_GRASS: [usize; 5] = [0, 5, 6, 3, 4];

pub struct StartMenu {
    pub minifont: FontHandle,
    pub position: SubpixelFancyPosition,
    pub grass: Grass,
    is_seeded: bool,
    pub velocity: FixedI16<8>,
}

impl StartMenu {
    pub(crate) fn init(console: &mut Console) -> StartMenu {
        let minifont = FontHandle::init(console, 0, gt_crust::system::console::SpriteRamQuadrant::One);

        Self {
            minifont,
            position: SubpixelFancyPosition {
                x: FixedU16::<8>::from_num(0),
                y: FixedU16::<8>::from_num(0),
            },
            grass: Grass {
                array: STARTING_GRASS,
            },
            is_seeded: false,
            velocity: FixedI16::<8>::from_num(1.3),
        }
    }


    fn draw_start_text(&mut self, ticks: u64, console: &mut Console) {
        // let y_offset = (ticks % (78)) / 26; // 3 states, 26 ticks long each
        let y_offset = 0u8;
        self.minifont.draw_string(console, 30, 80 - (y_offset as u8), &string_to_indices!("Press Start, Gamer"));
    }

}


impl GameState for StartMenu {
    fn update_and_draw(mut self, ticks: u64, console: &mut Console) -> GameStates {
        console.via.profiler_start(0);

        draw_background(console, true);

        draw_clouds(&self.position.to_fancy(), console);

        console.via.profiler_start(1);
        self.draw_start_text(ticks, console);
        console.via.profiler_end(1);


        console.via.profiler_start(2);
        self.grass.draw_grass(self.position.to_fancy(), console);
        console.via.profiler_end(2);

        if console.gamepad_1.is_pressed(Buttons::Start) {
            console.rng_seed = ticks;
            self.is_seeded = true;
        }

        self.position.x = self.position.x.add_signed(self.velocity);

        if self.is_seeded && ticks % 16 == 0 {
            return GameStates::Runup(Runup::init(self, console))
        }
        console.via.profiler_end(0);

        GameStates::StartMenu(self)
    }
}