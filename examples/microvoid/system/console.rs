use crate::system::bcr::Bcr;
use crate::system::scr::DmaLocation::Vram;
use crate::system::scr::MirroredScr;
use crate::system::vram::VramDma;
use wyhash;
use crate::system::inputs;
use crate::system::inputs::{GamePadPort, GameshockOne};

pub enum SpriteRamQuadrant {
    One,
    Two,
    Three,
    Four,
}

#[derive(PartialEq, Eq)]
pub enum BlitMode {
    Normal,
    FlipX,
    FlipY,
    FlipXY
}

impl SpriteRamQuadrant {
    fn value_gx(&self) -> u8 {
        match self {
            Self::One | Self::Three => 0,
            Self::Two | Self::Four => 128
        }
    }

    fn value_gy(&self) -> u8 {
        match self {
            Self::One | Self::Two => 0,
            Self::Three | Self::Four => 128
        }
    }
}

pub struct Sprite {
    pub(crate) bank: u8,
    pub(crate) vram_x: u8,
    pub(crate) vram_y: u8,
    pub(crate) width: u8,
    pub(crate) height: u8,
}

impl Sprite {
    pub fn draw_sprite(&self, x: u8, y: u8, blit_mode: BlitMode, console: &mut Console) {
        let (mut width, mut height) = (self.width, self.height);
        let (mut gx, mut gy) = (self.vram_x, self.vram_y);

        if blit_mode == BlitMode::FlipX || blit_mode == BlitMode::FlipXY {
            gx ^= 0xFF;
            gx -= self.width - 1;
            width ^= 0b10000000;
        }

        if blit_mode == BlitMode::FlipY || blit_mode == BlitMode::FlipXY {
            gy ^= 0xFF;
            gy -= self.height - 1;
            height ^= 0b10000000;
        }


        console.control_registers.set_dma_enable(true);

        console.control_registers.set_colorfill_mode(false);
        console.control_registers.set_vram_bank(self.bank);
        console.control_registers.set_dma_gcarry(true);

        console.blitter_registers.vram_x.write(gx);
        console.blitter_registers.vram_y.write(gy);
        console.blitter_registers.fb_x.write(x);
        console.blitter_registers.fb_y.write(y);
        console.blitter_registers.width.write(width);
        console.blitter_registers.height.write(height);
        console.blitter_registers.start.write(1);

        unsafe { gt_crust::boot::wait(); }
        console.blitter_registers.reset_irq();
    }

    pub fn draw_sprite_with_overscan(&self, mut x: i16, mut y: i16, blit_mode: BlitMode, console: &mut Console) {
        let (mut width, mut height) = (self.width, self.height);
        let (mut vram_x, mut vram_y) = (self.vram_x, self.vram_y);

        let (mut new_x, mut new_y) = (x as u8, y as u8);

        // don't render if off screen
        if x > 127 || y > 127 ||
            x <= -(width as i16) || y <= -(height as i16) {
            return
        }


        if x < 0 {
            width = (width as i16 + x) as u8;
            vram_x = (vram_x as i16 - x) as u8;
            new_x = 0;
        } else if width > (128 - x) as u8 {
            width = (128 - x) as u8;
        }


        // TODO: fix bug when overscanning near 0y with FlipY
        if y < 0 {
            height = (height as i16 + y) as u8;
            vram_y = (vram_y as i16 - y) as u8;
            new_y = 0;
        } else if height > (128 - y) as u8 {
            height = (128 - y) as u8;
        }

        if blit_mode == BlitMode::FlipX || blit_mode == BlitMode::FlipXY {
            vram_x ^= 0xFF;
            vram_x -= self.width - 1;
            width ^= 0b10000000;
        }

        if blit_mode == BlitMode::FlipY || blit_mode == BlitMode::FlipXY {
            vram_y ^= 0xFF;
            vram_y -= self.height - 1;
            height ^= 0b10000000;
        }


        console.control_registers.set_dma_enable(true);

        console.control_registers.set_colorfill_mode(false);
        console.control_registers.set_vram_bank(self.bank);
        console.control_registers.set_dma_gcarry(true);

        console.blitter_registers.vram_x.write(vram_x);
        console.blitter_registers.vram_y.write(vram_y);
        console.blitter_registers.fb_x.write(new_x);
        console.blitter_registers.fb_y.write(new_y);
        console.blitter_registers.width.write(width);
        console.blitter_registers.height.write(height);
        console.blitter_registers.start.write(1);

        unsafe { gt_crust::boot::wait(); }
        console.blitter_registers.reset_irq();
    }
}


/// the public friendly APIs?
pub struct Console {
    control_registers: MirroredScr,
    blitter_registers: &'static mut Bcr,
    vram: VramDma,
    pub gamepad_1: GameshockOne,
    pub gamepad_2: GameshockOne,
    // rng_seed: u64
}

impl Console {
    pub fn init() -> Console {
        let mut scr: MirroredScr = unsafe { MirroredScr::new() };
        let bcr:&mut Bcr = unsafe { Bcr::new() };

        scr.enable_vblank_nmi(true);

        Console {
            control_registers: scr,
            blitter_registers: bcr,
            vram: VramDma::new(),
            gamepad_1: GameshockOne::init(GamePadPort::One),
            gamepad_2: GameshockOne::init(GamePadPort::Two),
            // rng_seed: 42069
        }
    }


    pub fn access_vram_bank(&mut self, bank: u8, quadrant: &SpriteRamQuadrant) -> &mut VramDma {
        self.quadrant_select_blit(quadrant);
        self.control_registers.set_vram_bank(bank);
        self.control_registers.set_dma_location(Vram);
        self.control_registers.set_dma_enable(false);

        &mut self.vram
    }

    pub fn draw_box(&mut self, x:u8, y:u8, w:u8, h:u8, c:u8) {
        self.control_registers.set_dma_enable(true);

        self.control_registers.set_colorfill_mode(true);

        self.blitter_registers.fb_x.write(x);
        self.blitter_registers.fb_y.write(y);
        self.blitter_registers.width.write(w);
        self.blitter_registers.height.write(h);
        self.blitter_registers.color.write(c);
        self.blitter_registers.start.write(1);

        unsafe { gt_crust::boot::wait(); }
        self.blitter_registers.reset_irq();
    }

    pub fn quadrant_select_blit(&mut self, quad: &SpriteRamQuadrant) {
        self.control_registers.set_dma_enable(true);

        self.control_registers.set_colorfill_mode(false);

        self.blitter_registers.fb_x.write(128);
        self.blitter_registers.fb_y.write(128);
        self.blitter_registers.vram_x.write(quad.value_gx());
        self.blitter_registers.vram_y.write(quad.value_gy());
        self.blitter_registers.width.write(1);
        self.blitter_registers.height.write(1);
        self.blitter_registers.start.write(1);

        unsafe { gt_crust::boot::wait(); }
        self.blitter_registers.reset_irq();
    }

    pub fn await_vblank(&mut self) {
        self.control_registers.set_dma_enable(true);

        self.blitter_registers.reset_irq();
        unsafe { gt_crust::boot::wait(); }
    }

    pub fn flip_framebuffer(&mut self) {
        self.control_registers.flip_framebuffer();
    }
}
