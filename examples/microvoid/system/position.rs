use derive_more::{Add, AddAssign, Sub, SubAssign};
use fixed::types::extra::U8;

#[derive(Debug, Copy, Clone, Add, AddAssign, Sub, SubAssign)]
pub struct ScreenSpacePosition {
    pub x: u8,
    pub y: u8,
}

impl ScreenSpacePosition {
    pub fn to_fancy(&self) -> FancyPosition {
        FancyPosition {
            x: self.x + 64,
            y: self.y + 64,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SubpixelFancyPosition {
    pub x: fixed::FixedU16<U8>, // the position in "worldspace", -64 to 192
    pub y: fixed::FixedU16<U8>, // the position in "worldspace", -64 to 192
}

#[derive(Debug, Copy, Clone, Add, AddAssign, Sub, SubAssign)]
pub struct FancyPosition {
    pub x: u8, // the position in "worldspace", -64 to 192
    pub y: u8, // the position in "worldspace", -64 to 192
}

impl FancyPosition {
    #[inline]
    pub fn to_screenspace(&self) -> ScreenSpacePosition {
        ScreenSpacePosition {
            x: self.x - 64,
            y: self.y - 64,
        }
    }
}

pub struct Dimensions {
    pub width: u8,
    pub height: u8,
}

impl SubpixelFancyPosition {
    #[inline]
    pub fn to_fancy(&self) -> FancyPosition {
        FancyPosition {
            x: (self.x.round().to_bits() >> 8) as u8,
            y: (self.y.round().to_bits() >> 8) as u8,
        }
    }

    #[inline]
    pub fn to_screenspace(&self) -> ScreenSpacePosition {
        self.to_fancy().to_screenspace()
    }
}
