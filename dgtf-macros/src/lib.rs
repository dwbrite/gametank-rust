extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashMap;
use quote::quote;
use std::fs;
use tinybmp;
use tinybmp::ColorTable;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use colors_transform::*;
use serde::{Deserialize, Serialize};
use syn::{parse_macro_input, LitStr, punctuated::Punctuated, Token};
use syn::parse::Parser;
use syn::__private::TokenStreamExt;

mod bmp;


#[derive(Serialize, Deserialize, Debug)]
struct Frame {
    frame: FrameFrame
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
    frames: Vec<FrameFrame>
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
        struct SpriteSheet {
            pixels_per_byte: u8,
            palette: [u8; 16],
            width: u8,
            height: u8,
            pixel_array: [u8; #pixel_array_size],
        }

        static SPRITESHEET: SpriteSheet = SpriteSheet {
            pixels_per_byte: #pixels_per_byte,
            width: #width as u8,
            height: #height as u8,
            palette: [#(#palette),*],
            pixel_array: [#(#pixel_array),*],
        };

        &SPRITESHEET
    }
};

    output.into()
}
