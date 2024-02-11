use core::ops::Sub;
use fixed::{FixedI16, FixedU16};
use fixed::types::extra::U8;
use crate::font::FontHandle;
use crate::system::console::{Console};
use crate::aesthetic::background::{draw_background, draw_clouds};
use crate::aesthetic::grass::{Grass};
// use crate::bad_dudes::{BadDude, BadDudes, LineDude, NoDude};
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::runup::Runup;
use crate::gamer::*;
use crate::system::position::{f16u8_to_u8, FancyPosition, fu16u8_to_u8, ScreenSpacePosition, SubpixelFancyPosition};

pub struct Playing {
    minifont: FontHandle, // TODO: maybe abstract the fields/fns common between start_menu and playing
    position: SubpixelFancyPosition, // range of -64..192 as a 0.5+1.0+0.5 screen, recycled view
    gamer: Gamer,
    grass: Grass,
    score: u16,
    silly_digits: FixedU16<U8>,
    // dudes: [BadDudes; 3], // max 4 bad dudes on screen at a time, I guess?
    velocity: FixedI16<U8>,
    dead: u64,
}

impl Playing {
    pub(crate) fn init(runup: Runup) -> Playing{
        Self {
            minifont: runup.minifont,
            position: runup.position,
            score: 00000,
            grass: runup.grass,
            gamer: runup.gamer,
            silly_digits: FixedU16::<U8>::from_num(0),
            // dudes: [
            //     BadDudes::NoDude(NoDude {}),
            //     BadDudes::NoDude(NoDude {}),
            //     BadDudes::NoDude(NoDude {}),
            //     // BadDudes::NoDude(NoDude {})
            // ],
            // fuck it, we're targeting 30 FPS
            velocity: runup.velocity,
            dead: 0
        }
    }
}

// fn new_random_dude(console: &mut Console) -> BadDudes {
//     let rng = console.fast_rng();
//     let rng2 = console.fast_rng();
//
//     let height = (2 + (rng % 8)) * 2; // 4..=20
//
//     return BadDudes::LineDude(LineDude {
//         position: SubpixelFancyPosition { x: FixedU16::<U8>::from(64+128+(rng%32+rng2%64)), y: FixedU16::<U8>::from(100+64) },
//         height,
//     });
// }


impl GameState for Playing {
    fn update_and_draw(mut self, ticks: u64, console: &mut Console) -> GameStates {
        // draw_background(console, false);
        // draw_clouds(&self.position.to_fancy(), console);
        // self.grass.draw_grass(self.position.to_fancy(), console);
        // self.gamer.update_and_draw( self.velocity, console);
        //
        // self.minifont.draw_number(console, 0, 0, self.score, fu16u8_to_u8(self.silly_digits));
        //
        // let gamer_rect = self.gamer.to_rectangle();
        // // for dude in &mut self.dudes {
        // //     if dude.is_offscreen_left() {
        // //         *dude = new_random_dude(console);
        // //     }
        // //     dude.update_and_draw(self.velocity, console);
        // //
        // //     if gamer_rect.intersects(dude.hitbox()) {
        // //         if self.dead == 0 {
        // //             self.velocity = FixedI16::<U8>::from_num(1.5);
        // //         }
        // //
        // //         self.dead = 1;
        // //     }
        // // }
        //
        // if self.dead > 0 {
        //     if ticks % 10 == 0 {
        //         self.dead+=1;
        //         if self.velocity > FixedI16::<U8>::from_num(0.0) {
        //             self.velocity -= FixedI16::<U8>::from_num(0.125);
        //         }
        //     }
        //
        //     if self.gamer.subpixel_pos.to_screenspace().y == 100 {
        //         self.gamer.set_state(GamerStates::Sliding);
        //     }
        // }
        //
        // self.silly_digits = self.silly_digits.add_signed(self.velocity);
        //
        // if self.silly_digits >= 100 {
        //     self.score += 1;
        //     self.silly_digits = self.silly_digits.sub(FixedU16::<U8>::from_num(100));
        // }
        //
        // if self.score > 1 && self.score % 1000 == 0 {
        //     self.velocity += FixedI16::<U8>::from_num(0.125);
        // }
        //
        // // allow it to overflow
        // self.position.x = self.position.x.add_signed(self.velocity);

        GameStates::Playing(self)
    }
}