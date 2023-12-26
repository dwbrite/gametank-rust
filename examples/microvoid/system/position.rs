use core::ops;

#[derive(Debug, Copy, Clone)]
pub struct ScreenSpacePosition {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct SubpixelFancyPosition {
    pub x: u8, // the position in "worldspace", -64 to 192
    pub y: u8, // the position in "worldspace", -64 to 192
    pub fraction_x: u8, // subpixel position - used for physics
    pub fraction_y: u8, // subpixel position - used for physics
}

#[derive(Debug, Copy, Clone)]
pub struct FancyPosition {
    pub x: u8, // the position in "worldspace", -64 to 192
    pub y: u8, // the position in "worldspace", -64 to 192
}

impl FancyPosition {
    pub fn to_screenspace(&self) -> ScreenSpacePosition {
        ScreenSpacePosition {
            x: self.x - 64,
            y: self.y - 64,
        }
    }
}

impl ops::Add<FancyPosition> for FancyPosition {
    type Output = FancyPosition;

    fn add(self, _rhs: FancyPosition) -> FancyPosition {
        FancyPosition {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::AddAssign<FancyPosition> for FancyPosition {
    fn add_assign(&mut self, rhs: FancyPosition) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl ops::SubAssign<FancyPosition> for FancyPosition {
    fn sub_assign(&mut self, rhs: FancyPosition) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl ops::Sub<FancyPosition> for FancyPosition {
    type Output = FancyPosition;

    fn sub(self, _rhs: FancyPosition) -> FancyPosition {
        FancyPosition {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

pub struct Dimensions {
    pub width: u8,
    pub height: u8,
}

impl SubpixelFancyPosition {
    pub fn to_fancy_position(&self) -> FancyPosition {
        FancyPosition {
            x: self.x,
            y: self.y,
        }
    }
}
