use crate::font::FontHandle;
use crate::system::console::{Console};
use crate::aesthetic::background::{draw_background};
use crate::aesthetic::grass::{Grass};
use crate::bad_dudes::{BadDude, BadDudes, LineDude, NoDude};
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::runup::Runup;
use crate::gamer::*;
use crate::system::position::{FancyPosition, ScreenSpacePosition};

pub struct Playing {
    minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    position: FancyPosition, // range of -64..192 as a 0.5+1.0+0.5 screen, recycled view
    gamer: Gamer,
    grass: Grass,
    score: u16,
    silly_digits: u8,
    dudes: [BadDudes; 4], // max 4 bad dudes on screen at a time, I guess?
    bbbb: u16,
}

impl Playing {
    pub(crate) fn init(runup: Runup) -> Playing{
        Self {
            minifont: runup.minifont,
            position: runup.position,
            score: 00000,
            grass: runup.grass,
            gamer: runup.gamer,
            silly_digits: 00,
            dudes: [
                BadDudes::NoDude(NoDude {}),
                BadDudes::NoDude(NoDude {}),
                BadDudes::NoDude(NoDude {}),
                BadDudes::NoDude(NoDude {})
            ],
            bbbb: 0xbbbb,
        }
    }
}


impl GameState for Playing {
    fn update_and_draw(mut self, ticks: u64, console: &mut Console) -> GameStates {
        draw_background(console, false);
        // draw_clouds(&self.position, console);
        self.grass.draw_grass(self.position, console);
        self.gamer.update_and_draw(console);

        self.minifont.draw_number(console, 0, 0, self.score, self.silly_digits);

        if self.position.x == 0 || self.position.x == 128 {
            let working_dude = &self.dudes[0];
            let rng = console.fast_rng();

            if working_dude.is_offscreen_left() {
                let height = (2 + (rng % 7)) * 2; // between 2 and 9 inclusive, *2
                self.bbbb = height as u16 + ((rng as u16) << 8);
                self.dudes[0] = BadDudes::LineDude(LineDude {
                    position: ScreenSpacePosition { x: 110, y: 100 }.to_fancy(),
                    height,
                });
            }
        }

        for dude in &mut self.dudes {
            dude.update_and_draw(ticks, console);
        }

        // self.gamer.to_rectangle().draw(console);

        self.silly_digits += 1;

        if self.silly_digits >= 100 {
            self.score += 1;
            self.silly_digits -= 100;
        }

        // allow it to overflow
        self.position.x += 1;

        GameStates::Playing(self)
    }
}