// tests/test_my_macro.rs

use std::ops::Deref;
use dgtf_macros::include_spritesheet;
use dgtf_macros::string_to_indices;

include_spritesheet!(MY_SPRITESHEET, "assets/minifont-p.bmp", "assets/minifont-p.json");


#[test]
fn test_my_macro() {
    let spritesheet = &MY_SPRITESHEET;

    println!("{:?}, {}", spritesheet, std::mem::size_of_val(spritesheet));

    let abc = string_to_indices!("ABC abc \"hi wrld\"");

    println!("{:?}", abc);
}
