#![no_std]
#![no_main]
#![feature(generic_const_exprs)]
#![feature(const_fn_floating_point_arithmetic)]

extern crate gt_crust;

mod font;
mod gamestates;
mod stuff;
mod aesthetic;
mod gamer;
mod bad_dudes;


use crate::gamestates::{GameState, GameStates};
use crate::stuff::{load_assorted_sprites};
use gt_crust::system::console::*;
use crate::gamestates::runup::Runup;

#[no_mangle]
#[link_section = ".text.fixed"]
fn main() {
    let mut console = Console::init();
    load_assorted_sprites(&mut console);
    // load_title_sprite(&mut console);

    let mut ticks = 0u64;

    // keeping gamestates very light, so we don't overflow the stack
    let mut current_state: GameStates = GameStates::Runup(Runup::init(&mut console));

    loop {
        console.await_vblank();
        console.flip_framebuffer();

        console.gamepad_1.read();
        current_state = current_state.update_and_draw(ticks, &mut console);

        console.draw_box(0,0,1,127, 0b00011111, false);
        console.draw_box(0,127,1,1, 0b00011111, false);
        console.draw_box(127,0,1,127, 0b00011111, false);
        console.draw_box(127,127,1,1, 0b00011111, false);

        ticks+=1; // do we _really_ need to?...
    }
}
