extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use std::fs;



use colors_transform::*;
use serde::{Deserialize, Serialize};
use syn::{parse_macro_input, LitStr, Token};
use syn::parse::Parser;


mod bmp;


#[derive(Serialize, Deserialize, Debug)]
struct Frame {
    frame: FrameFrame,
    spriteSourceSize: FrameFrame,
}

#[derive(Serialize, Deserialize, Debug)]
struct FrameFrame {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct FrameData {
    frames: Vec<Frame>
}


#[derive(Debug)]
struct Sprite {
    sheet_x: u8,
    sheet_y: u8,
    width: u8,
    height: u8,
    x_offset: u8,
    y_offset: u8,
    // TODO: We'll probably want more at some point. Maybe.
}


#[proc_macro]
pub fn include_spritesheet(input: TokenStream) -> TokenStream {
    // Convert the input TokenStream to a TokenStream2 for parsing
    let input = proc_macro2::TokenStream::from(input);

    // Define a parser for a tuple of two string literals
    let parser = syn::punctuated::Punctuated::<LitStr, Token![,]>::parse_separated_nonempty;
    let args = parser.parse2(input).expect("Expected two string literals separated by a comma, paths to bmp and json files for a spritesheet");

    let mut iter = args.into_iter();
    let bmp_path = iter.next().expect("Expected first string literal").value();
    let json_path = iter.next().expect("Expected second string literal").value();

    let json_file_contents = fs::read_to_string(json_path)
        .expect("Failed to read JSON file");

    let frame_data: FrameData = serde_json::from_str(&json_file_contents).expect("Failed to parse JSON");

    let mut sprites = vec![];
    let num_sprites = frame_data.frames.len();

    for frame in frame_data.frames {
        sprites.push(Sprite {
            sheet_x: frame.frame.x,
            sheet_y: frame.frame.y,
            width: frame.frame.w,
            height: frame.frame.h,
            x_offset: frame.spriteSourceSize.x,
            y_offset: frame.spriteSourceSize.y,
        });
    }

    let sprite_tokens: Vec<_> = sprites.iter().map(|sprite| {
        let sheet_x = sprite.sheet_x;
        let sheet_y = sprite.sheet_y;
        let width = sprite.width;
        let height = sprite.height;
        let x_offset = sprite.x_offset;
        let y_offset = sprite.y_offset;

        quote! {
            Sprite {
                sheet_x: #sheet_x,
                sheet_y: #sheet_y,
                width: #width,
                height: #height,
                x_offset: #x_offset,
                y_offset: #y_offset,
            }
        }
    }).collect();

    let spritesheetimage = bmp::SpriteSheetImage::load_spritesheet(bmp_path);

    let pixels_per_byte = spritesheetimage.pixels_per_byte;
    let width = spritesheetimage.width;
    let height = spritesheetimage.height;
    let palette = spritesheetimage.palette;
    let pixel_array = spritesheetimage.pixel_array;
    let pixel_array_size = pixel_array.len();

    let output = quote! {
    {
        #[derive(Debug, Copy, Clone)]
        struct Sprite {
            sheet_x: u8,
            sheet_y: u8,
            width: u8,
            height: u8,
            x_offset: u8,
            y_offset: u8,
        }

        #[derive(Debug, Copy, Clone)]
        struct SpriteSheet {
            pixels_per_byte: u8,
            palette: [u8; 16],
            width: u8,
            height: u8,
            sprite_data: [Sprite; #num_sprites],
            pixel_array: [u8; #pixel_array_size],
        }

        static SPRITESHEET: SpriteSheet = SpriteSheet {
            pixels_per_byte: #pixels_per_byte,
            width: #width as u8,
            height: #height as u8,
            palette: [#(#palette),*],
            sprite_data: [#(#sprite_tokens),*],
            pixel_array: [#(#pixel_array),*],
        };

        &SPRITESHEET
    }
};

    output.into()
}

#[proc_macro]
pub fn string_to_indices(input: TokenStream) -> TokenStream {
    let input_string = parse_macro_input!(input as LitStr).value();
    let characters = " ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                      abcdefghijklmnopqrstuvwxyz\
                      1234567890\
                      !?.,;:/\"\"()[]{}<>"; // Removed the quote marks from here

    let mut quote_count = 0;
    let left_quote_index = characters.find("\"").unwrap();
    let right_quote_index = left_quote_index + 1; // Index for the right quote

    let indices: Vec<usize> = input_string.chars()
        .map(|c| {
            if c == '"' {
                quote_count += 1;
                if quote_count % 2 == 0 { right_quote_index } else { left_quote_index }
            } else {
                characters.find(c).unwrap_or_else(|| panic!("Character '{}' not found", c))
            }
        })
        .collect();

    let output = quote! {
        [ #( #indices ),* ]
    };

    output.into()
}

