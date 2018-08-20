extern crate failure;
extern crate ndimage;

use ndimage::core::{
    padding::Padding, DynamicImage, Image2D, ImageBuffer2D, Pixel, PixelCast, Primitive
};
use ndimage::io::{open, png::PngEncodable, save};
use ndimage::processing::kernel::Kernel;

use failure::Error;

use std::env::{args, current_dir};

fn apply_convolutions<'a, P, T, Pf>(img: &'a Image2D<P>) -> Result<(), Error>
where
    P: Pixel<Subpixel = T> + PixelCast<f64, Output = Pf> + PngEncodable<P>,
    Pf: Pixel<Subpixel = f64> + PixelCast<T, Output = P>,
    T: Primitive
{
    let box_kernel = Kernel::<f64>::box_(6);
    let gaussian_kernel = Kernel::gaussian(3., 6);
    let box_img: ImageBuffer2D<P> = box_kernel.convolve(img, Padding::Replicate);
    let gaussian_img: ImageBuffer2D<P> = gaussian_kernel.convolve(img, Padding::Replicate);

    let mut box_file = current_dir()?;
    let mut gaussian_file = current_dir()?;
    box_file.push("box.png");
    gaussian_file.push("gaussian.png");

    save(box_file, &box_img)?;
    save(gaussian_file, &gaussian_img)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let in_img = args().nth(1).unwrap();

    let in_img_dyn = open(in_img)?;
    match in_img_dyn {
        DynamicImage::RgbU8(img) => apply_convolutions(img.as_ref()),
        DynamicImage::LumaU8(img) => apply_convolutions(img.as_ref()),
        _ => {
            println!("Only 8 bit RGB and grayscale images are supported.");
            Ok(())
        }
    }
}
