use crate::system::bcr::Bcr;
use crate::system::scr::DmaLocation::Vram;
use crate::system::scr::MirroredScr;
use crate::system::vram::VramDma;
use wyhash;
use wyhash::wyrng;
use crate::system::inputs;
use crate::system::inputs::{GamePadPort, GameshockOne};

#[derive(Debug, Copy, Clone)]
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


/// the public friendly APIs?
pub struct Console {
    pub(crate) control_registers: MirroredScr,
    pub(crate) blitter_registers: &'static mut Bcr,
    vram: VramDma,
    pub gamepad_1: GameshockOne,
    pub gamepad_2: GameshockOne,
    rng_seed: u64 // TODO: make an optional struct for rng?
    // TODO: maybe generate a bunch of rng
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
            rng_seed: 69420
        }
    }

    pub fn preseed_rng(&mut self, seed: u64) {
        self.rng_seed = seed
    }

    pub fn rng(&mut self) -> u64 {
        wyrng(&mut self.rng_seed)
    }


    pub fn access_vram_bank(&mut self, bank: u8, quadrant: &SpriteRamQuadrant) -> &mut VramDma {
        self.quadrant_select_blit(quadrant);
        self.control_registers.set_vram_bank(bank);
        self.control_registers.set_dma_location(Vram);
        self.control_registers.set_dma_enable(false);

        &mut self.vram
    }

    pub fn draw_box(&mut self, x:u8, y:u8, w:u8, h:u8, c:u8) {
        // while self.blitter_registers.start.read() != 0 {}

        self.control_registers.set_dma_enable(true);

        self.control_registers.set_colorfill_mode(true);

        self.blitter_registers.fb_x.write(x);
        self.blitter_registers.fb_y.write(y);
        self.blitter_registers.width.write(w);
        self.blitter_registers.height.write(h);
        self.blitter_registers.color.write(c);
        self.blitter_registers.start.write(1);

        //
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

        // while self.blitter_registers.start.read() == 1 {}
        unsafe { gt_crust::boot::wait(); }
        self.blitter_registers.reset_irq();
    }

    pub fn await_vblank(&mut self) {
        self.control_registers.set_dma_enable(true);

        self.blitter_registers.reset_irq(); // set to 0
        unsafe { gt_crust::boot::wait(); }
    }

    pub fn flip_framebuffer(&mut self) {
        self.control_registers.flip_framebuffer();
    }
}
