// use core::ops::Div;
use fixed::{FixedI16, FixedU16};

use crate::gamer::GamerStates::{ControlledFalling, Jumping, Running, Sliding, Standing, UncontrolledFalling};
use gt_crust::system::{sprite};
use gt_crust::system::console::{BlitMode, Console, SpriteRamQuadrant};
use gt_crust::system::inputs::Buttons;
use gt_crust::system::position::{FancyPosition, SubpixelFancyPosition};
use gt_crust::system::rectangle::Rectangle;
use gt_crust::system::sprite::VramBank;

// creates a Sprite and SpriteSheet struct in this module, as well as a static SpriteSheet GAMER_SPRITES
dgtf_macros::include_spritesheet!(GAMER_SPRITES, "examples/microvoid/assets/gamer_con_polvo.bmp", "examples/microvoid/assets/gamer_con_polvo.json");

// pub const FRAME_TIMES: [u8; 12] =   [0,  5,  5,  5,  5,  5,  5,  5,  4,  0,  0,  0]; // this was 3x what it should be at 60fps???
pub const X_OFFSET: [u8; 12] =      [2,  0,  1,  2,  2,  0,  2,  2,  2,  2,  2,  6];
pub const Y_OFFSET: [u8; 12] =      [0,  0,  0,  0,  1,  0,  0,  0,  0,  0,  0,  2];

pub const FRAME_RUN_DISTANCE: [FixedI16<8>; 12] = [
    FixedI16::<8>::lit("0"),
    FixedI16::<8>::lit("6.65"),
    FixedI16::<8>::lit("6.65"),
    FixedI16::<8>::lit("6.65"),
    FixedI16::<8>::lit("6.65"),
    FixedI16::<8>::lit("6.65"),
    FixedI16::<8>::lit("6.65"),
    FixedI16::<8>::lit("6.65"),
    FixedI16::<8>::lit("5.4"),
    FixedI16::<8>::lit("00"),
    FixedI16::<8>::lit("00"),
    FixedI16::<8>::lit("00"),
];

pub const BASE_GRAVITY: FixedI16<8> = FixedI16::<8>::lit("0.28");
pub const INITIAL_JUMP_GRAVITY: FixedI16<8> = FixedI16::<8>::lit("0.07");
pub const CONTROLLED_FALL_GRAVITY: FixedI16<8> = FixedI16::<8>::lit("0.045");
pub const MAX_VELOCITY: FixedI16<8> = FixedI16::<8>::lit("2.5");
pub const ZERO: FixedI16<8> = FixedI16::<8>::lit("0.0");
pub const JUMP_VELOCITY: FixedI16<8> = FixedI16::<8>::lit("-2.1");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamerStates {
    Standing,
    Running,
    Jumping,
    ControlledFalling,
    UncontrolledFalling,
    Sliding,
}

impl GamerStates {
    const fn to_animation_idx(&self) -> usize {
        match self {
            Standing => 0,
            Running => 1,
            Jumping => 9,
            ControlledFalling => 10,
            UncontrolledFalling => 10,
            Sliding => 11
        }
    }
}


pub struct Gamer {
    pub bank: u8,
    pub quadrant: SpriteRamQuadrant,
    pub spritesheet: &'static GAMER_SPRITES_SpriteSheet,
    pub frame_counter: u8,
    pub animation: usize,
    pub state: GamerStates,
    pub subpixel_pos: SubpixelFancyPosition,
    pub holding_jump: bool,
    pub velocity: FixedI16<8>,
    pub acceleration: FixedI16<8>,
    pub no_jump: u8, // reused for jump timing
    pub run_frame_position: FixedI16<8>, // bruh.png
}

impl Gamer {
    pub fn init(console: &mut Console, bank: u8, quadrant: SpriteRamQuadrant) -> Self {
        let sprite_sheet = &GAMER_SPRITES;
        let vram = console.access_vram_bank(bank, &quadrant);

        let bits_per_pixel = 8 / sprite_sheet.pixels_per_byte as usize;
        let mask = (1 << bits_per_pixel) - 1;

        // TODO: probably extract this lol
        for byte_index in 0..sprite_sheet.pixel_array.len() {
            let byte = sprite_sheet.pixel_array[byte_index];

            for idx_idx in 0..(sprite_sheet.pixels_per_byte as usize) {
                let pixel_index = (byte >> (bits_per_pixel * idx_idx)) & mask;
                let color = sprite_sheet.palette[pixel_index as usize];

                let overall_pixel_index = byte_index * sprite_sheet.pixels_per_byte as usize + idx_idx;

                vram.memory[overall_pixel_index].write(color);
            }
        }

        for y in 0..sprite_sheet.height as usize {
            for x in 0..sprite_sheet.width as usize {
                let input = x + y * sprite_sheet.width as usize;
                let output = x + (y + 40) * 128;

                vram.memory[output].write(vram.memory[input].read());
            }
        }

        Self {
            bank,
            quadrant,
            spritesheet: sprite_sheet,
            frame_counter: 0,
            animation: UncontrolledFalling.to_animation_idx(),
            state: UncontrolledFalling,
            subpixel_pos: SubpixelFancyPosition {
                x: FixedU16::<8>::from_num(24+64),
                y: FixedU16::<8>::from_num(0+64-10),
            },
            holding_jump: false,
            velocity: ZERO,
            acceleration: ZERO,
            no_jump: 0,
            run_frame_position: ZERO,
        }
    }

    pub fn set_animation(&mut self, anim: usize) {
        self.animation = anim
    }

    pub fn set_state(&mut self, state: GamerStates) {
        self.state = state;
        self.frame_counter = 0;
        self.run_frame_position = ZERO;
        self.animation = state.to_animation_idx();
    }

    pub fn sim_air_physics(&mut self) {
        self.velocity += self.acceleration;

        if self.velocity > MAX_VELOCITY {
            self.velocity = MAX_VELOCITY;
        }

        self.subpixel_pos.y = self.subpixel_pos.y.add_signed(self.velocity);

        if self.subpixel_pos.to_fancy().y > 100+64 {
            self.subpixel_pos.y = FixedU16::<8>::from(64+100);
            self.velocity = ZERO;
            self.acceleration = ZERO;
            self.set_state(Running);
            self.no_jump = 0; // no_jump frames are CANCELLED
        }
    }

    pub fn update_and_draw(&mut self, velocity_x: FixedI16<8>, console: &mut Console) {
        if self.holding_jump && !console.gamepad_1.is_pressed(Buttons::A) {
            self.holding_jump = false;
        }

        match self.state {
            Running => {
                if self.no_jump == 0 && console.gamepad_1.is_pressed(Buttons::A) {
                    self.holding_jump = true;
                    self.set_state(Jumping);
                    self.velocity = JUMP_VELOCITY;
                    self.no_jump = 1;
                }

                if self.no_jump > 0 {
                    self.no_jump -= 1;
                }
            }
            Jumping => {
                if self.holding_jump && !console.gamepad_1.is_pressed(Buttons::A) {
                    self.holding_jump = false;
                }

                self.acceleration = BASE_GRAVITY;
                if self.holding_jump && self.no_jump < 10 {
                    self.acceleration = INITIAL_JUMP_GRAVITY;
                }

                self.sim_air_physics();


                if self.velocity > ZERO {
                    if self.holding_jump {
                        self.set_state(ControlledFalling);
                    } else {
                        self.set_state(UncontrolledFalling);
                    }
                }
            }
            ControlledFalling  => {
                self.acceleration = CONTROLLED_FALL_GRAVITY;

                self.sim_air_physics();

                if !self.holding_jump  {
                    self.set_state(UncontrolledFalling);
                }
            }
            UncontrolledFalling => {
                self.acceleration = BASE_GRAVITY;
                self.sim_air_physics();
            }
            _ => { /* not implemented */ }
        }


        let sprite_data = self.spritesheet.sprite_data[self.animation];
        let sprite = sprite::Sprite {
            bank: VramBank {
                bank: self.bank,
                quadrant: self.quadrant.clone(),
            },
            vram_x: sprite_data.sheet_x, // TODO: add quadrant, never use "hardware coords" for addressing vram
            vram_y: sprite_data.sheet_y + 40,
            width: sprite_data.width,
            height: sprite_data.height,
            is_tile: false,
        };

        // origin is bottom-left ish
        let mut animation_offsets = FancyPosition {
            x: X_OFFSET[self.animation],
            y: Y_OFFSET[self.animation] + sprite.height,
        };

        if self.animation == Sliding.to_animation_idx() {
            animation_offsets.y -= 4;
        }

        let position = self.subpixel_pos.to_fancy() - animation_offsets;

        sprite.draw_sprite_with_overscan(position, BlitMode::Normal, console);

        self.frame_counter += 1;

        // TODO: this could _probably_ be improved, right?

        self.run_frame_position += velocity_x;

        if self.run_frame_position > FRAME_RUN_DISTANCE[self.animation] {
            self.run_frame_position -= FRAME_RUN_DISTANCE[self.animation].clone();
            match self.animation {
                1..=7 => self.animation += 1,
                8 => self.animation = 1,
                _ => {}
            }
        }
    }

    pub fn to_rectangle(&self) -> Rectangle {
        let height = self.spritesheet.sprite_data[self.animation].height;
        let mut rect_xy = self.subpixel_pos.to_fancy();
        rect_xy.y -= height - 1 + Y_OFFSET[self.animation];
        rect_xy.x +=1;

        Rectangle {
            xy: rect_xy,
            size: FancyPosition {
                x: 6,
                y: 8,
            }
        }
    }
}