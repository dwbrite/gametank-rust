use crate::system::bcr::Bcr;
use crate::system::scr::DmaLocation::Vram;
use crate::system::scr::MirroredScr;
use crate::system::vram::VramDma;
use wyhash;
use wyhash::wyrng;
use crate::system::inputs::{GamePadPort, GameshockOne};
use crate::system::via::Via;

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
    pub via: &'static mut Via,
    vram: VramDma,
    pub gamepad_1: GameshockOne,
    pub gamepad_2: GameshockOne,
    pub rng_seed: u64, // TODO: make an optional struct for rng?
    pub rng: u64,
    rng_idx: usize,
    // TODO: maybe generate a bunch of rng
}

impl Console {
    #[link_section = ".text.fixed"]
    pub fn init() -> Console {
        let mut scr: MirroredScr = unsafe { MirroredScr::new() };
        let bcr:&mut Bcr = unsafe { Bcr::new() };
        let via = unsafe { Via::new() };

        scr.enable_vblank_nmi(true);

        Console {
            control_registers: scr,
            blitter_registers: bcr,
            via,
            vram: VramDma::new(),
            gamepad_1: GameshockOne::init(GamePadPort::One),
            gamepad_2: GameshockOne::init(GamePadPort::Two),
            rng_seed: 69_420,
            rng: 80085_69_420,
            rng_idx: 0,
        }
    }

    pub fn preseed_rng(&mut self, seed: u64) {
        self.rng_seed = seed
    }

    pub fn rng(&mut self) -> u64 {
        self.rng = wyrng(&mut self.rng_seed);
        self.rng_idx = 0;
        self.rng
    }

    pub fn fast_rng(&mut self) -> u8 {
        let result = self.rng.to_ne_bytes()[self.rng_idx];
        self.rng_idx += 1;

        // ohhhhh, this totally was oob indexing before, lol
        if self.rng_idx >= 8 {
            // red box to let the programmer know they
            // FUCKED UP and used more than 8 bytes of rng without refilling >:(
            // self.draw_box(0,0,69,69, 0b100_00_110, false);
            // that should generate RNG, but in case it didn't, do it AGAIN >:(((
            self.rng();
        }

        result
    }


    pub fn access_vram_bank(&mut self, bank: u8, quadrant: &SpriteRamQuadrant) -> &mut VramDma {
        self.quadrant_select_blit(quadrant);
        self.control_registers.set_vram_bank(bank);
        self.control_registers.set_dma_location(Vram);
        self.control_registers.set_dma_enable(false);

        &mut self.vram
    }

    pub fn draw_box(&mut self, x:u8, y:u8, w:u8, h:u8, c:u8, gen_rng: bool) {
        self.control_registers.set_dma_enable(true);

        self.control_registers.set_colorfill_mode(true);

        self.blitter_registers.fb_x.write(x);
        self.blitter_registers.fb_y.write(y);
        self.blitter_registers.width.write(w);
        self.blitter_registers.height.write(h);
        self.blitter_registers.color.write(c);
        self.blitter_registers.start.write(1);

        if gen_rng {
            self.rng();
        }

        //
        unsafe { gt_crust::boot::wait(); }
        self.blitter_registers.reset_irq();
    }

    pub fn quadrant_select_blit(&mut self, quad: &SpriteRamQuadrant) {
        self.control_registers.set_dma_enable(true);

        // TODO: WAS THIS THIS CULPRIT ALL ALONG?!?!?!?!?!
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
        self.control_registers.enable_vblank_nmi(true);

        self.blitter_registers.reset_irq(); // set to 0 -- is this necessary?
        unsafe { gt_crust::boot::wait(); }
    }

    pub fn flip_framebuffer(&mut self) {
        self.control_registers.flip_framebuffer();
    }
}
