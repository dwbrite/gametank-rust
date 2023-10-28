// #![allow(dead_code)]
// #![no_std]
//
// #[repr(C,packed)]
// pub struct Scr {
//     audio_reset: Volatile<u8>,
//     audio_nmi: Volatile<u8>,
//     _pad0: [u8; 3], // Skips to $2005
//     banking: Volatile<u8>,
//     audio_reg: Volatile<u8>,
//     video_reg: Volatile<u8>,
// }
//
// impl Scr {
//     // Creates System Control Register at the proper address
//     pub unsafe fn new() -> &'static mut Scr {
//         &mut *(0x2000 as *mut Scr)
//     }
//
//     pub fn flip_framebuffer(&mut self) {
//         self.video_reg.write(self.video_reg.read() ^ 0b00000010);
//     }
// }

pub fn test_function() {}

pub fn do_it_up() {
    // let mut scr = unsafe { Scr::new() };
    loop {
        unsafe {
            // scr.flip_framebuffer();
        }
    }
}
