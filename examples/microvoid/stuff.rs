use gt_crust::system::console::{Console, SpriteRamQuadrant};
dgtf_macros::include_spritesheet!(ASSORTED_SPRITES, "examples/microvoid/assets/other_stuff.bmp", "examples/microvoid/assets/other_stuff.json");


pub fn load_assorted_sprites(console: &mut Console) {
    let sprite_sheet = &ASSORTED_SPRITES;

    let vram = console.access_vram_bank(0, &SpriteRamQuadrant::Two);

    let bits_per_pixel = 8 / sprite_sheet.pixels_per_byte as usize;
    let mask = (1 << bits_per_pixel) - 1;

    for byte_index in 0..sprite_sheet.pixel_array.len() {
        let byte = sprite_sheet.pixel_array[byte_index];

        for idx_idx in 0..(sprite_sheet.pixels_per_byte as usize) {
            let pixel_index = (byte >> (bits_per_pixel * idx_idx)) & mask;
            let color = sprite_sheet.palette[pixel_index as usize];

            let overall_pixel_index = byte_index * sprite_sheet.pixels_per_byte as usize + idx_idx;

            vram.memory[overall_pixel_index].write(color);
        }
    }

    for y in 0..sprite_sheet.height as usize {
        for x in 0..sprite_sheet.width as usize {
            let input = x+y*sprite_sheet.width as usize;
            let output = x + (y+40)*128;

            vram.memory[output].write(vram.memory[input].read());
        }
    }
}

