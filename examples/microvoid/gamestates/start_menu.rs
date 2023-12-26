use crate::font::FontHandle;
use crate::system::console::{BlitMode, Console, Sprite, SpriteRamQuadrant};
use dgtf_macros::string_to_indices;
use crate::aesthetic::grass::Grass;
use crate::gamestates::{GameState, GameStates};
use crate::gamestates::runup::Runup;
use crate::system::inputs::Buttons;

pub const STARTING_GRASS: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 5, 3, 6, 2];

pub struct StartMenu {
    pub minifont: FontHandle,
    pub position: i16, // range of -128..255 as a "3 pane" recycled view
    pub grass: Grass,
    is_seeded: bool,
}

impl StartMenu {
    pub(crate) fn init(mut console: &mut Console) -> StartMenu {
        let minifont = FontHandle::init(&mut console, 0, crate::system::console::SpriteRamQuadrant::One);

        Self {
            minifont,
            position: 0,
            grass: Grass {
                array: STARTING_GRASS,
            },
            is_seeded: false,
        }
    }

    pub(crate) fn draw_background(&self, console: &mut Console) {
        console.draw_box(0, 0, 127, 100, 0b101_00_000);
        console.draw_box(127, 0, 1, 100, 0b101_00_000);
        console.draw_box(0, 100, 127, 28, 0b011_10_110);
        console.draw_box(127, 100, 1, 28, 0b011_10_110);
    }

    fn draw_start_text(&mut self, ticks: u64, mut console: &mut Console) {
        let y_offset = (ticks % (78)) / 26; // 3 states, 26 ticks long each
        self.minifont.draw_string(&mut console, 30, 80 - (y_offset as u8), &string_to_indices!("Press Start, Gamer"));
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
}


impl GameState for StartMenu {
    fn update_and_draw(mut self, ticks: u64, mut console: &mut Console) -> GameStates {
        self.draw_background(console);
        self.draw_clouds(console);
        self.draw_start_text(ticks, console);
        self.grass.draw_grass(self.position, console);


        if self.is_seeded && ticks % 16 == 0 {
            return GameStates::Runup(Runup::init(self, &mut console))
        }

        if console.gamepad_1.is_pressed(&Buttons::Start) {
            console.preseed_rng(ticks);
            self.is_seeded = true;
        }

        self.position += 1;

        if self.position >= (128*3) {
            self.position = 0;
        }

        GameStates::StartMenu(self)
    }
}