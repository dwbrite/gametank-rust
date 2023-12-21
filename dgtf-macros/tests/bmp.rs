// tests/test_my_macro.rs

use dgtf_macros::include_spritesheet;

#[test]
fn test_my_macro() {
    let spritesheet = include_spritesheet!("assets/minifont-sp.bmp");

    println!("{}", spritesheet);
}