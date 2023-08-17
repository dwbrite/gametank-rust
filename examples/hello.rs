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
        &mut *(0x2000 as *mut Scr)
    }

    pub fn flip_framebuffer(&mut self) {
        self.video_reg.write(self.video_reg.read() ^ 0b00000010);
    }
}

#[no_mangle]
fn main() {
    let mut scr = unsafe { Scr::new() };
    loop {
        unsafe {
            scr.flip_framebuffer();
            my_function();
        }
    }
}