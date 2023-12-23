use crate::font::FontHandle;
use crate::system::console::{BlitMode, Console, Sprite};
use dgtf_macros::string_to_indices;

#[enum_delegate::implement(GameState)]
pub enum GameStates {
    StartMenu(StartMenu),
}

#[enum_delegate::register]
pub trait GameState {
    fn update_and_draw(&mut self, ticks: u64, console: &mut Console);
}

pub struct StartMenu {
    minifont: FontHandle,
    position: i16, // range of -128..255 as a "3 pane" recycled view
}

impl StartMenu {
    pub(crate) fn init(mut console: &mut Console) -> StartMenu {
        let minifont = FontHandle::init(&mut console, 0, crate::system::console::SpriteRamQuadrant::One);

        Self {
            minifont,
            position: 0,
        }
    }

    fn draw_start_text(&mut self, ticks: u64, mut console: &mut Console) {
        let y_offset = (ticks % (78)) / 26;
        self.minifont.draw_string(&mut console, 27, 80 - y_offset as u8, &string_to_indices!("Press Start to Game"));
    }

    fn yeet(self) -> FontHandle {
        self.minifont
    }
}

fn draw_clouds(ticks: u64, mut console: &mut Console) {

}

impl GameState for StartMenu {
    fn update_and_draw(&mut self, ticks: u64, mut console: &mut Console) {
        console.draw_box(0, 0, 127, 100, 0b101_00_000);
        console.draw_box(127, 0, 1, 100, 0b101_00_000);
        console.draw_box(0, 100, 127, 28, 0b011_10_110);
        console.draw_box(127, 100, 1, 28, 0b011_10_110);


        self.draw_start_text(ticks, console);

        self.position += 1;

        if self.position >= (128*3) {
            self.position = 0;
        }

        // draw cloud
        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[3];
        let sprite = Sprite {
            bank: 0,
            vram_x: sprite_data.sheet_x + 128, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
        };

        sprite.draw_sprite_with_overscan(128 - self.position, 15, BlitMode::Normal, &mut console);

    }
}