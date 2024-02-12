use crate::system::console::Console;
use crate::system::position::FancyPosition;

pub struct Rectangle {
    pub xy: FancyPosition, // struct { x: u8, y: u8 }
    pub size: FancyPosition,
}

impl Rectangle {
    pub fn intersects(&self, other: Rectangle) -> bool {
        // Check if one rectangle is to the left of the other
        if self.xy.x + self.size.x <= other.xy.x || other.xy.x + other.size.x <= self.xy.x {
            return false;
        }

        // Check if one rectangle is above the other
        if self.xy.y + self.size.y <= other.xy.y || other.xy.y + other.size.y <= self.xy.y {
            return false;
        }

        // Rectangles intersect
        true
    }

    pub fn draw(&self, console: &mut Console) {
        // TODO: draw box with overscan
        let screen_xy = self.xy.to_screenspace();
        console.draw_box(screen_xy.x, screen_xy.y, self.size.x, self.size.y, 0b100_00_011, false);
    }
}