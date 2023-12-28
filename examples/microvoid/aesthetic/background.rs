use crate::system::console::{BlitMode, Console, SpriteRamQuadrant};
use crate::system::position::FancyPosition;
use crate::system::sprite::{Sprite, VramBank};

pub fn draw_background(console: &mut Console, redraw_ground: bool) {
    console.draw_box(0, 0, 127, 100, 0b101_00_000, true);
    console.draw_box(127, 0, 1, 100, 0b101_00_000, false);

    if redraw_ground {
        console.draw_box(0, 100, 127, 28, 0b011_10_110, false);
        console.draw_box(127, 100, 1, 28, 0b011_10_110, false);
    }
}

pub fn draw_clouds(position: &FancyPosition, mut console: &mut Console) {
    let cloud_1 = FancyPosition {
        x: 64u8.wrapping_sub(position.x),
        y: 73,
    };

    let cloud_2 = FancyPosition {
        x: 32+192u8.wrapping_sub(position.x), // just off screen
        y: 94,
    };

    let sprite_data = crate::stuff::ASSORTED_SPRITES.sprite_data[3];
    let sprite = Sprite {
        bank: VramBank {
            bank: 0,
            quadrant: SpriteRamQuadrant::Two,
        },
        vram_x: sprite_data.sheet_x,
        vram_y: sprite_data.sheet_y + 40,
        width: sprite_data.width,
        height: sprite_data.height,
        is_tile: false,
        with_interrupt: false,
    };

    sprite.draw_sprite_with_overscan(cloud_1, BlitMode::Normal, &mut console);
    sprite.draw_sprite_with_overscan(cloud_2, BlitMode::Normal, &mut console);
}