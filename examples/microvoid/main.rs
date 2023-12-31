#![no_std]
#![no_main]

extern crate gt_crust;

pub mod system;
mod font;
mod gamestates;
mod stuff;
mod aesthetic;
mod gamer;
mod bad_dudes;

use crate::gamestates::{GameState, GameStates};
use crate::gamestates::start_menu::StartMenu;
use crate::stuff::load_assorted_sprites;
use crate::system::console::*;

#[no_mangle]
fn main() {
    let mut console = Console::init();
    load_assorted_sprites(&mut console);

    let mut ticks = 0u64;

    // keeping gamestates very light, so we don't overflow the stack
    let mut current_state: GameStates = GameStates::StartMenu(StartMenu::init(&mut console));

    loop {
        console.await_vblank();
        console.flip_framebuffer();

        console.gamepad_1.read();
        current_state = current_state.update_and_draw(ticks, &mut console);

        ticks+=1; // do we _really_ need to?...
    }
}
