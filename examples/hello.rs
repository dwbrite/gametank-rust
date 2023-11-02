#![no_std]
#![no_main]

extern crate gt_crust;

use core;
use volatile::Volatile;
use bit_field::BitField;

#[link(name = "my_function", kind = "static")]
extern "C" {
    fn my_function();
}

/// System Control Register
/// $2000 	Write 1 to reset audio coprocessor
/// $2001 	Write 1 to send NMI to audio coprocessor
/// $2005 	Banking Register
/// $2006 	Audio enable and sample rate
/// $2007 	Video/Blitter Flags
#[repr(C,packed)]
pub struct Scr {
    audio_reset_mirror: Volatile<u8>,
    audio_nmi_mirror: Volatile<u8>,
    _pad0_mirror: [u8; 3], // Skips to $2005
    banking_mirror: Volatile<u8>,
    audio_reg_mirror: Volatile<u8>,
    video_reg_mirror: Volatile<u8>,
/////////////// straddle the $2000 line between RAM and SCRs
    audio_reset: Volatile<u8>,
    audio_nmi: Volatile<u8>,
    _pad0: [u8; 3], // Skips to $2005
    banking: Volatile<u8>,
    audio_reg: Volatile<u8>,
    video_reg: Volatile<u8>,
}

impl Scr {
    // Creates System Control Register at the proper address
    pub unsafe fn new() -> &'static mut Scr {
        let _scr = &mut *(0x1FF8 as *mut Scr);
        _scr.banking.write(0);
        _scr.banking_mirror.write(8);
        _scr.video_reg_mirror.write(69);
        _scr.video_reg.write(_scr.video_reg_mirror.read());
        _scr.banking.write(_scr.banking_mirror.read());
        _scr
    }

    pub fn flip_framebuffer(&mut self) {
        self.video_reg_mirror.write(self.video_reg_mirror.read() ^ 0b00000010);
        self.banking_mirror.write(self.banking_mirror.read() ^ 0b00001000);
        self.video_reg.write(self.video_reg_mirror.read());
        self.banking.write(self.banking_mirror.read());
    }

    pub fn set_colorfill_mode(&mut self) {
        self.video_reg_mirror.write(self.video_reg_mirror.read() | 0b00001000);
        self.video_reg.write(self.video_reg_mirror.read());
    }

    pub fn unset_colorfill_mode(&mut self) {
        self.video_reg_mirror.write(self.video_reg_mirror.read() & 0b11110111);
        self.video_reg.write(self.video_reg_mirror.read());
    }
}

/*
#define vram_VX ((volatile char *) 0x4000)
#define vram_VY ((volatile char *) 0x4001)
#define vram_GX ((volatile char *) 0x4002)
#define vram_GY ((volatile char *) 0x4003)
#define vram_WIDTH ((volatile char *) 0x4004)
#define vram_HEIGHT ((volatile char *) 0x4005)
#define vram_START ((volatile char *) 0x4006)
#define vram_COLOR ((volatile char *) 0x4007)
*/
#[repr(C,packed)]
pub struct Bcr {
    VX: Volatile<u8>,
    VY: Volatile<u8>,
    GX: Volatile<u8>,
    GY: Volatile<u8>,
    WIDTH: Volatile<u8>,
    HEIGHT: Volatile<u8>,
    START: Volatile<u8>,
    COLOR: Volatile<u8>,
}

impl Bcr {
    //Blitter control registers
    pub unsafe fn new() -> &'static mut Bcr {
        &mut *(0x4000 as *mut Bcr)
    }

    pub fn draw_box(&mut self, _scr:&mut Scr, x:u8, y:u8, w:u8, h:u8, c:u8) {
        _scr.set_colorfill_mode();
        self.VX.write(x);
        self.VY.write(y);
        self.WIDTH.write(w);
        self.HEIGHT.write(h);
        self.COLOR.write(c);
        self.START.write(1);
    }
}

#[no_mangle]
fn main() {
    let mut scr:&mut Scr = unsafe { Scr::new() };
    let mut bcr:&mut Bcr = unsafe { Bcr::new() };
    let mut ball_x:u8 = 10;
    let mut ball_y:u8 = 10;
    let mut dx:u8 = 1;
    let mut dy:u8 = 1;

    loop {
        unsafe {
            my_function();
            scr.flip_framebuffer();
            bcr.draw_box(scr, 0, 0, 127, 127, 92);
            my_function();
            bcr.draw_box(scr, ball_x, ball_y, 16, 16, 3);
            my_function();
            ball_x += dx;
            ball_y += dy;
            if(ball_x == 111) {dx = 255;}
            if(ball_x == 1) {dx = 1;}
            if(ball_y == 84) {dy = 255;}
            if(ball_y == 1) {dy = 1;}
        }
    }
}