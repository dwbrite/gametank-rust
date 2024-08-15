use crate::gamestates::playing::Playing;
use crate::gamestates::runup::Runup;
// use crate::gamestates::start_menu::StartMenu;
use gt_crust::system::console::Console;

mod playing;
pub mod runup;

#[enum_delegate::implement(GameState)]
pub enum GameStates {
    // StartMenu(StartMenu),
    Runup(Runup),
    Playing(Playing),
}

#[enum_delegate::register]
pub trait GameState {
    fn update_and_draw(self, ticks: u64, console: &mut Console) -> GameStates;
}