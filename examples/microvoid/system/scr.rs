use volatile::Volatile;
use bit_field::BitField;
use crate::system::console::SpriteRamQuadrant;

pub enum DmaLocation {
    Vram,
    Framebuffer
}

impl DmaLocation {
    pub fn value(self) -> bool {
        match self {
            DmaLocation::Vram => false,
            DmaLocation::Framebuffer => true,
        }
    }
}

/// System Control Register
/// $2000 	Write 1 to reset audio coprocessor
/// $2001 	Write 1 to send NMI to audio coprocessor
/// $2005 	Banking Register
/// $2006 	Audio enable and sample rate
/// $2007 	Video/Blitter Flags
#[repr(C,packed)]
pub struct  Scr {
    audio_reset: Volatile<u8>,
    audio_nmi: Volatile<u8>,
    _pad0: [u8; 3], // Skips to $2005
    banking: Volatile<u8>,
    audio_reg: Volatile<u8>,
    video_reg: Volatile<u8>,
}

pub struct MirroredScr {
    scr: &'static mut Scr,
    mirror: Scr
}

impl MirroredScr {
    /// 0 = copy 16x16 across whole buffer
    pub(crate) fn set_dma_gcarry(&mut self, gcarry: bool) {
        self.mirror.video_reg.update(|val| *val = *val.set_bit(4, gcarry));
        self.scr.video_reg.write(self.mirror.video_reg.read());
    }

    pub fn set_vram_bank(&mut self, bank: u8) {
        self.mirror.banking.update(|val| *val = *val.set_bits(0..3, bank));
        self.scr.banking.write(self.mirror.banking.read());
    }

    pub unsafe fn new() -> MirroredScr {
        let s = MirroredScr {
            scr: &mut *(0x2000 as *mut Scr),
            mirror: Scr {
                audio_reset: Volatile::new(0),
                audio_nmi: Volatile::new(0),
                _pad0: [0; 3], // Skips to $2005
                banking: Volatile::new(8),
                audio_reg: Volatile::new(0),
                video_reg: Volatile::new(69), // nice
            }
        };

        s.scr.audio_reset.write(s.mirror.audio_reset.read());
        s.scr.audio_nmi.write(s.mirror.audio_nmi.read());
        s.scr.banking.write(s.mirror.banking.read());
        s.scr.audio_reg.write(s.mirror.audio_reg.read());
        s.scr.video_reg.write(s.mirror.video_reg.read());
        s
    }

    pub fn flip_framebuffer(&mut self) {
        self.mirror.video_reg.write(self.mirror.video_reg.read() ^ 0b00000010);
        self.mirror.banking.write(self.mirror.banking.read() ^ 0b00001000);
        self.scr.video_reg.write(self.mirror.video_reg.read());
        self.scr.banking.write(self.mirror.banking.read());
    }

    pub fn set_colorfill_mode(&mut self, enable: bool) {
        self.mirror.video_reg.update(|val| *val = *val.set_bit(3, enable));
        self.scr.video_reg.write(self.mirror.video_reg.read());
    }

    pub fn enable_vblank_nmi(&mut self, enable: bool) {
        self.mirror.video_reg.update(|val| *val = *val.set_bit(2, enable));
        self.scr.video_reg.write(self.mirror.video_reg.read());
    }

    /// set dma enabled - 0 is DMA enabled, 1 allows access to blitter commands.
    /// TODO correct, rename, whatever, do something to unfuck this mess.
    pub fn set_dma_enable(&mut self, enable: bool) {
        self .mirror.video_reg.update(|val| *val = *val.set_bit(0, enable));
        self.scr.video_reg.write(self.mirror.video_reg.read());
    }

    pub fn set_dma_location(&mut self, location: DmaLocation) {
        self.mirror.video_reg.update(|val| *val = *val.set_bit(5, location.value()));
        self.scr.video_reg.write(self.mirror.video_reg.read());
    }
}