use crate::font::FontHandle;
use crate::system::console::{BlitMode, Console, Sprite, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::system::inputs::Buttons;

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
    clyde_x: i16,
    clyde_y: i16,
}

impl StartMenu {
    pub(crate) fn init(mut console: &mut Console) -> StartMenu {
        let minifont = FontHandle::init(&mut console, 0, crate::system::console::SpriteRamQuadrant::One);

        Self {
            minifont,
            position: 0,
            clyde_y: 25,
            clyde_x: 25
        }
    }

    pub(crate) fn draw_background(&self, console: &mut Console) {
        console.draw_box(0, 0, 127, 100, 0b101_00_000);
        console.draw_box(127, 0, 1, 100, 0b101_00_000);
        console.draw_box(0, 100, 127, 28, 0b011_10_110);
        console.draw_box(127, 100, 1, 28, 0b011_10_110);
    }

    fn draw_start_text(&mut self, ticks: u64, mut console: &mut Console) {
        let y_offset = (ticks % (78)) / 26;
        self.minifont.draw_string(&mut console, 27, 80 - y_offset as u8, &string_to_indices!("Press Start to Game"));
    }

    fn draw_clouds(&mut self, mut console: &mut Console) {
        let cloud_1 = (128, 9);
        let cloud_2 = (256+32, 30);

        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[3];
        let sprite = Sprite {
            bank: 0,
            vram_x: sprite_data.sheet_x + 128, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
        };

        sprite.draw_sprite_with_overscan(cloud_1.0 - self.position, cloud_1.1, BlitMode::Normal, &mut console);
        sprite.draw_sprite_with_overscan(cloud_2.0 - self.position, cloud_2.1, BlitMode::Normal, &mut console);
    }

    fn yeet(self) -> FontHandle {
        self.minifont
    }
}


impl GameState for StartMenu {
    fn update_and_draw(&mut self, ticks: u64, mut console: &mut Console) {
        self.draw_background(console);
        self.draw_background(console);
        self.draw_clouds(console);
        self.draw_start_text(ticks, console);

        if ticks % 3 == 0 {
            if console.gamepad_1.is_pressed(&Buttons::Left) {
                self.clyde_x -= 1;
            }
            if console.gamepad_1.is_pressed(&Buttons::Right) {
                self.clyde_x += 1;
            }
            if console.gamepad_1.is_pressed(&Buttons::Up) {
                self.clyde_y -= 1;
            }
            if console.gamepad_1.is_pressed(&Buttons::Down) {
                self.clyde_y += 1;
            }
        }

        let i = if console.gamepad_1.is_pressed(&Buttons::C) {
            5
        } else {
            4
        };

        let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[i];
        let sprite = Sprite {
            bank: 0,
            vram_x: sprite_data.sheet_x + 128, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
        };

        if console.gamepad_1.is_pressed(&Buttons::A) {
            sprite.draw_sprite_with_overscan(self.clyde_x, self.clyde_y, BlitMode::FlipX, console);
        } else if console.gamepad_1.is_pressed(&Buttons::B) {
            sprite.draw_sprite_with_overscan(self.clyde_x, self.clyde_y, BlitMode::Normal, console);
        }


        self.position += 1;

        if self.position >= (128*3) {
            self.position = 0;
        }
    }
}