use crate::Bcr;
use crate::MirroredScr;
use crate::system::scr::DmaLocation::Vram;
use crate::system::vram::VramDma;

pub enum SpriteRamQuadrant {
    One,
    Two,
    Three,
    Four,
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


/// the public friendly APIs?
pub struct Console {
    control_registers: MirroredScr,
    blitter_registers: &'static mut Bcr,
    vram: VramDma
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
        }
    }


    pub fn access_vram_bank(&mut self, bank: u8, quadrant: SpriteRamQuadrant) -> &mut VramDma {
        self.quadrant_select_blit(quadrant);
        self.control_registers.set_vram_bank(bank);
        self.control_registers.set_dma_location(Vram);
        self.control_registers.set_dma_enable(false);

        &mut self.vram
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, x: u8, y: u8) {
        self.control_registers.set_dma_enable(true);

        self.control_registers.set_colorfill_mode(false);
        self.control_registers.set_vram_bank(sprite.bank);

        self.blitter_registers.gx.write(sprite.vram_x);
        self.blitter_registers.gy.write(sprite.vram_y);
        self.blitter_registers.vx.write(x);
        self.blitter_registers.vy.write(y);
        self.blitter_registers.width.write(sprite.width);
        self.blitter_registers.height.write(sprite.height);
        self.blitter_registers.start.write(1);

        unsafe { gt_crust::boot::wait(); }
        self.blitter_registers.reset_irq();
    }

    pub fn draw_box(&mut self, x:u8, y:u8, w:u8, h:u8, c:u8) {
        self.control_registers.set_dma_enable(true);

        self.control_registers.set_colorfill_mode(true);

        self.blitter_registers.vx.write(x);
        self.blitter_registers.vy.write(y);
        self.blitter_registers.width.write(w);
        self.blitter_registers.height.write(h);
        self.blitter_registers.color.write(c);
        self.blitter_registers.start.write(1);

        unsafe { gt_crust::boot::wait(); }
        self.blitter_registers.reset_irq();
    }

    pub fn quadrant_select_blit(&mut self, quad: SpriteRamQuadrant) {
        self.control_registers.set_dma_enable(true);

        self.control_registers.set_colorfill_mode(false);

        self.blitter_registers.vx.write(128);
        self.blitter_registers.vy.write(128);
        self.blitter_registers.gx.write(quad.value_gx());
        self.blitter_registers.gy.write(quad.value_gy());
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
