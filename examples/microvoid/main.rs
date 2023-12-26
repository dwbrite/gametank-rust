#![no_std]
#![no_main]

extern crate gt_crust;

pub mod system;
mod font;
mod gamestates;
mod stuff;

use crate::font::FontHandle;
use crate::gamestates::{GameState, GameStates, StartMenu};
use crate::stuff::load_assorted_sprites;
use crate::system::console::*;

#[no_mangle]
fn main() {
    let mut console = Console::init();
    load_assorted_sprites(&mut console);

    let mut ticks = 0u64;

    let mut current_state: GameStates = GameStates::StartMenu(StartMenu::init(&mut console));

    loop {
        console.await_vblank();
        console.flip_framebuffer();

        console.gamepad_1.read();
        current_state = current_state.update_and_draw(ticks, &mut console);

        ticks+=1; // do we _really_ need to?...
    }
}
