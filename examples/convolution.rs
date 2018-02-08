extern crate ndimage;

use ndimage::kernel::Kernel;
use ndimage::io::png::{PngDecoder, PngEncoder8};
use ndimage::processing::histogram::equalize_histogram;

use std::env::{args, current_dir};
use std::fs::File;

fn main() {
    let in_img = args().nth(1).unwrap();
    let mut box_img = current_dir().unwrap();
    let mut gaussian_img = current_dir().unwrap();
    let mut equalized_img = current_dir().unwrap();
    box_img.push("box.png");
    gaussian_img.push("gaussian.png");
    equalized_img.push("equalized.png");
    let in_file = File::open(in_img).unwrap();
    let box_file = File::create(box_img).unwrap();
    let gaussian_file = File::create(gaussian_img).unwrap();
    let equalized_file = File::create(equalized_img).unwrap();

    let decoder = PngDecoder::new(&in_file).unwrap();
    let img = decoder.read_luma_u8().unwrap();

    let box_kernel = Kernel::<f64>::box_(6);
    let gaussian_kernel = Kernel::gaussian(3., 6);
    let box_img = box_kernel.convolve(&img);
    let gaussian_img = gaussian_kernel.convolve(&img);
    let equalized = equalize_histogram(&img);

    let encoder = PngEncoder8::new(&box_img, box_file).unwrap();
    encoder.write().unwrap();
    let encoder = PngEncoder8::new(&gaussian_img, gaussian_file).unwrap();
    encoder.write().unwrap();
    let encoder = PngEncoder8::new(&equalized, equalized_file).unwrap();
    encoder.write().unwrap();
}