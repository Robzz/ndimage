#![feature(test)]

extern crate image;
extern crate ndimage;
extern crate test;

use test::Bencher;

const W: u32 = 1920;
const H: u32 = 1080;

#[cfg(test)]
mod bench_ndimage {
    use super::*;
    use ndimage::core::{Image2D, Image2DMut, ImageBuffer2D, Luma, Rect};

    #[bench]
    fn ndimage_fill_gray(b: &mut Bencher) {
        let mut img = ImageBuffer2D::<Luma<u8>>::new(W, H);
        b.iter(|| {
            img.fill(&Luma::new([127]));
        });
    }

    #[bench]
    fn ndimage_fill_loop_gray(b: &mut Bencher) {
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
        let mut img = ImageBuffer2D::<Luma<u8>>::new(W, H);
        b.iter(|| {
            for pix in &mut img {
                pix.data[0] = 127;
            }
        });
    }

    #[bench]
    fn ndimage_fill_rect_iter_gray(b: &mut Bencher) {
        let r = Rect::new(320, 180, 1280, 720);
        let mut img = ImageBuffer2D::<Luma<u8>>::new(W, H);
        b.iter(|| {
            for pix in img.rect_iter_mut(r) {
                pix.data[0] = 127;
            }
        });
    }

    #[bench]
    fn ndimage_fill_sub_image_gray(b: &mut Bencher) {
        let r = Rect::new(320, 180, 1280, 720);
        let mut img = ImageBuffer2D::<Luma<u8>>::new(W, H);
        b.iter(|| {
            for mut pix in &mut img.sub_image_mut(r) {
                pix.data[0] = 127;
            }
        });
        for y in 0..H {
            for x in 0..W {
                if x < 320 || y < 180 || x >= 1600 || y >= 900 {
                    assert_eq!(img.get_pixel(x, y), &Luma::new([0]));
                } else {
                    assert_eq!(img.get_pixel(x, y), &Luma::new([127]));
                }
            }
        }
    }
}

#[cfg(test)]
mod bench_image {
    use super::*;
    use image::{GenericImage, GrayImage, Luma};

    #[bench]
    fn image_fill_loop_gray(b: &mut Bencher) {
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
    fn image_fill_loop_gray_unsafe(b: &mut Bencher) {
        let mut img = GrayImage::new(W, H);
        b.iter(|| {
            for y in 0..H {
                for x in 0..W {
                    unsafe {
                        img.unsafe_put_pixel(x, y, Luma { data: [127] });
                    }
                }
            }
        });
    }

    #[bench]
    fn image_fill_iter_gray(b: &mut Bencher) {
        let mut img = GrayImage::new(W, H);
        b.iter(|| {
            for pix in img.pixels_mut() {
                pix.data[0] = 127;
            }
        });
    }

    #[bench]
    #[allow(deprecated)]
    fn image_fill_sub_image_gray(b: &mut Bencher) {
        let mut img = GrayImage::new(W, H);
        b.iter(|| {
            for (_x, _y, pix) in img.sub_image(320, 180, 1280, 720).pixels_mut() {
                pix.data[0] = 127;
            }
        });
        for y in 0..H {
            for x in 0..W {
                if x < 320 || y < 180 || x >= 1600 || y >= 900 {
                    assert_eq!(img.get_pixel(x, y), &Luma { data: [0u8] });
                } else {
                    assert_eq!(img.get_pixel(x, y), &Luma { data: [127u8] });
                }
            }
        }
    }
}
