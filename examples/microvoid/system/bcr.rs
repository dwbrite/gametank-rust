use volatile::Volatile;
use crate::MirroredScr;

#[repr(C,packed)]
pub struct Bcr {
    pub vx: Volatile<u8>,
    pub vy: Volatile<u8>,
    pub gx: Volatile<u8>,
    pub gy: Volatile<u8>,
    pub width: Volatile<u8>,
    pub height: Volatile<u8>,
    pub start: Volatile<u8>,
    pub color: Volatile<u8>,
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


    pub fn reset_irq(&mut self) {
        self.start.write(0);
    }
}