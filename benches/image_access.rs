#![feature(test)]

extern crate image;
extern crate ndimage;
extern crate test;

use test::Bencher;

#[cfg(test)]
mod bench_ndimage {
    use super::*;
    use ndimage::core::{Image2DMut, ImageBuffer2D, Luma};

    #[bench]
    fn ndimage_fill_loop_gray(b: &mut Bencher) {
        const W: u32 = 1280;
        const H: u32 = 720;
        let mut img = ImageBuffer2D::new(W, H);
        b.iter(|| {
            for y in 0..H {
                for x in 0..W {
                    img.put_pixel(x, y, Luma::new([127]));
                }
            }
        });
    }

    #[bench]
    fn ndimage_fill_iter_gray(b: &mut Bencher) {
        const W: u32 = 1280;
        const H: u32 = 720;
        let mut img = ImageBuffer2D::<Luma<u8>>::new(W, H);
        b.iter(|| {
            for pix in &mut img {
                pix.data[0] = 127;
            }
        });
    }

    #[bench]
    fn ndimage_fill_gray(b: &mut Bencher) {
        const W: u32 = 1280;
        const H: u32 = 720;
        let mut img = ImageBuffer2D::<Luma<u8>>::new(W, H);
        b.iter(|| {
            img.fill(Luma::new([127]));
        });
    }
}

#[cfg(test)]
mod bench_image {
    use super::*;
    use image::{GrayImage, Luma};

    #[bench]
    fn image_fill_loop_gray(b: &mut Bencher) {
        const W: u32 = 1280;
        const H: u32 = 720;
        let mut img = GrayImage::new(W, H);
        b.iter(|| {
            for y in 0..H {
                for x in 0..W {
                    img.put_pixel(x, y, Luma { data: [127] });
                }
            }
        });
    }

    #[bench]
    fn ndimage_fill_iter_gray(b: &mut Bencher) {
        const W: u32 = 1280;
        const H: u32 = 720;
        let mut img = GrayImage::new(W, H);
        b.iter(|| {
            for pix in img.pixels_mut() {
                pix.data[0] = 127;
            }
        });
    }
}
