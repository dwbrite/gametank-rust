#![no_std]
#![no_main]

extern crate gt_crust;

use volatile::Volatile;
use bit_field::BitField;

/// System Control Register
/// $2000 	Write 1 to reset audio coprocessor
/// $2001 	Write 1 to send NMI to audio coprocessor
/// $2005 	Banking Register
/// $2006 	Audio enable and sample rate
/// $2007 	Video/Blitter Flags
#[repr(C,packed)]
pub struct Scr {
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
        s.scr.audio_nmi.write(s.mirror.audio_reset.read());
        s.scr.banking.write(s.mirror.audio_reset.read());
        s.scr.audio_reg.write(s.mirror.audio_reset.read());
        s.scr.video_reg.write(s.mirror.audio_reset.read());
        s
    }

    pub fn flip_framebuffer(&mut self) {
        self.mirror.video_reg.write(self.mirror.video_reg.read() ^ 0b00000010);
        self.mirror.banking.write(self.mirror.banking.read() ^ 0b00001000);
        self.scr.video_reg.write(self.mirror.video_reg.read());
        self.scr.banking.write(self.mirror.banking.read());
    }

    pub fn set_colorfill_mode(&mut self) {
        self.mirror.video_reg.write(self.mirror.video_reg.read() | 0b00001000);
        self.scr.video_reg.write(self.mirror.video_reg.read());
    }

    pub fn unset_colorfill_mode(&mut self) {
        self.mirror.video_reg.write(self.mirror.video_reg.read() & 0b11110111);
        self.scr.video_reg.write(self.mirror.video_reg.read());
    }

    pub fn enable_vblank_nmi(&mut self, enable: bool) {
        let mut value = self.mirror.video_reg.read();
        value.set_bit(2, enable); // Set or clear the third least significant bit.
        self.scr.video_reg.write(value);
    }
}

#[repr(C,packed)]
pub struct Bcr {
    vx: Volatile<u8>,
    vy: Volatile<u8>,
    gx: Volatile<u8>,
    gy: Volatile<u8>,
    width: Volatile<u8>,
    height: Volatile<u8>,
    start: Volatile<u8>,
    color: Volatile<u8>,
}

/// Blitter Control Registers
/// vram_VX 0x4000
/// vram_VY 0x4001
/// vram_GX 0x4002
/// vram_GY 0x4003
/// vram_WIDTH 0x4004
/// vram_HEIGHT 0x4005
/// vram_START 0x4006
/// vram_COLOR 0x4007
impl Bcr {
    pub unsafe fn new() -> &'static mut Bcr {
        &mut *(0x4000 as *mut Bcr)
    }

    pub fn draw_box(&mut self, _scr: &mut MirroredScr, x:u8, y:u8, w:u8, h:u8, c:u8) {
        _scr.set_colorfill_mode();
        self.vx.write(x);
        self.vy.write(y);
        self.width.write(w);
        self.height.write(h);
        self.color.write(c);
        self.start.write(1);
    }


    pub fn reset_irq(&mut self) {
        self.start.write(0);
    }
}

#[no_mangle]
fn main() {
    let mut scr: MirroredScr = unsafe { MirroredScr::new() };
    let bcr:&mut Bcr = unsafe { Bcr::new() };
    let mut ball_x: i8 = 10;
    let mut ball_y: i8 = 10;
    let mut dx: i8 = 2;
    let mut dy: i8 = 2;

    scr.enable_vblank_nmi(true);

    unsafe { loop {
        bcr.reset_irq();
        gt_crust::boot::wait(); // wait for next interrupt (vblank nmi)

        scr.flip_framebuffer();

        bcr.draw_box(&mut scr, 0, 0, 127, 127, 0b010_00_100);
        gt_crust::boot::wait();
        bcr.reset_irq();

        bcr.draw_box(&mut scr, 0, 100, 127, 27, 0b111_10_101);
        gt_crust::boot::wait();
        bcr.reset_irq();

        bcr.draw_box(&mut scr, ball_x as u8, ball_y as u8, 6, 6, 0b100_00_010);
        gt_crust::boot::wait();
        bcr.reset_irq();

        if ball_x >= 121 { dx = -1; }
        if ball_x <= 0   { dx = 1; }

        if ball_y >= 94  { dy = -1; }
        if ball_y <= 0   { dy = 1; }

        ball_x += dx;
        ball_y += dy;
    }}
}