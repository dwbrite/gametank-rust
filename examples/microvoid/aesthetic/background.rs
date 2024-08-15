use gt_crust::system::console::{Console};



pub fn draw_background(console: &mut Console, redraw_ground: bool) {
    console.draw_box(1, 0, 126, 100, 0b101_00_000, false);

    if redraw_ground {
        console.draw_box(1, 100, 126, 28, 0b011_10_110, false);
    }
}
