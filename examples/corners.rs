//! Detect the corners in an image, mark them by a cross, and save the output image.

#[macro_use]
extern crate failure;
extern crate ndimage;

use ndimage::{
    core::{
        color_convert::{FromColor, Linear, Luma as PLuma},
        DynamicImage, Luma, Rgb
    },
    draw::draw_cross,
    features::harris::harris_corners,
    io::{open, save}
};

use failure::Error;

use std::env::args;

fn main() -> Result<(), Error> {
    let in_img_path = args().nth(1).unwrap();
    let out_img_path = args().nth(2).unwrap();
    let in_img_dyn = open(in_img_path).unwrap();
    match in_img_dyn {
        DynamicImage::RgbU8(mut img) => {
            let lin = Linear::<Rgb<u8>>::new();
            let luma = PLuma::<u8>::new();
            let img_gray = luma.from_image(&lin, img.as_ref());

            let corners = harris_corners(&img_gray, 1, 0.015);
            println!("Found {} corners", corners.len());
            for corner in corners {
                draw_cross(&mut *img, corner, 2, Rgb::new([255u8, 0u8, 0u8]));
            }
            save(out_img_path, img.as_ref()).unwrap();
        }
        DynamicImage::LumaU8(mut img) => {
            let corners = harris_corners(img.as_ref(), 1, 0.01);
            println!("Found {} corners", corners.len());
            for corner in corners {
                draw_cross(img.as_mut(), corner, 2, Luma::new([255u8]));
            }
            save(out_img_path, img.as_ref()).unwrap();
        }
        _ => {
            bail!("Unsupported image type!");
        }
    }
    Ok(())
}
