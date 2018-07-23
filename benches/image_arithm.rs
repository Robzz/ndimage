#![feature(test)]

extern crate image;
extern crate ndimage;
extern crate num_traits;
extern crate rand;
extern crate test;

mod bench_arithm {

const W: u32 = 1920;
const H: u32 = 1080;

use num_traits::{Bounded, One};
use rand::{thread_rng, Rng, distributions::Uniform};
use test::Bencher;

#[cfg(test)]
mod bench_ndimage {
    use super::*;
    use ndimage::core::{Image2D, Image2DMut, ImageBuffer2D, Luma, Rect, Pixel};

    macro_rules! gen_benches {
        ($name:ident, $pix:ty) => {
            mod $name {
                use super::*;

                #[bench]
                fn ndimage_add_imagebuffer2d(b: &mut Bencher) {
                    let img1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    let img2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    b.iter(|| {
                        let _ = (&img1 + &img2).unwrap();
                    });
                }

                #[bench]
                fn ndimage_add_image2d(b: &mut Bencher) {
                    let imgbuf1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    let imgbuf2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());

                    let img1: &Image2D<_> = &imgbuf1;
                    let img2: &Image2D<_> = &imgbuf2;
                    b.iter(|| {
                        let _ = (img1 + img2).unwrap();
                    });
                }
                #[bench]
                fn ndimage_sub_imagebuffer2d(b: &mut Bencher) {
                    let img1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    let img2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    b.iter(|| {
                        let _ = (&img1 - &img2).unwrap();
                    });
                }

                #[bench]
                fn ndimage_sub_image2d(b: &mut Bencher) {
                    let imgbuf1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    let imgbuf2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());

                    let img1: &Image2D<_> = &imgbuf1;
                    let img2: &Image2D<_> = &imgbuf2;
                    b.iter(|| {
                        let _ = (img1 - img2).unwrap();
                    });
                }
                #[bench]
                fn ndimage_mul_imagebuffer2d(b: &mut Bencher) {
                    let img1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    let img2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    b.iter(|| {
                        let _ = (&img1 * &img2).unwrap();
                    });
                }

                #[bench]
                fn ndimage_mul_image2d(b: &mut Bencher) {
                    let imgbuf1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                    let imgbuf2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());

                    let img1: &Image2D<_> = &imgbuf1;
                    let img2: &Image2D<_> = &imgbuf2;
                    b.iter(|| {
                        let _ = (img1 * img2).unwrap();
                    });
                }
                #[bench]
                fn ndimage_div_imagebuffer2d(b: &mut Bencher) {
                    let d = Uniform::new(<<$pix as Pixel>::Subpixel as Bounded>::min_value() + <<$pix as Pixel>::Subpixel as One>::one(), <<$pix as Pixel>::Subpixel as Bounded>::max_value());
                    let img1 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                    let img2 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                    b.iter(|| {
                        let _ = (&img1 / &img2).unwrap();
                    });
                }

                #[bench]
                fn ndimage_div_image2d(b: &mut Bencher) {
                    let d = Uniform::new(<<$pix as Pixel>::Subpixel as Bounded>::min_value() + <<$pix as Pixel>::Subpixel as One>::one(), <<$pix as Pixel>::Subpixel as Bounded>::max_value());
                    let imgbuf1 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                    let imgbuf2 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                    let img1: &Image2D<_> = &imgbuf1;
                    let img2: &Image2D<_> = &imgbuf2;
                    b.iter(|| {
                        let _ = (img1 / img2).unwrap();
                    });
                }

                #[bench]
                fn ndimage_rem_imagebuffer2d(b: &mut Bencher) {
                    let d = Uniform::new(<<$pix as Pixel>::Subpixel as Bounded>::min_value() + <<$pix as Pixel>::Subpixel as One>::one(), <<$pix as Pixel>::Subpixel as Bounded>::max_value());
                    let img1 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                    let img2 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                    b.iter(|| {
                        let _ = (&img1 % &img2).unwrap();
                    });
                }

                #[bench]
                fn ndimage_rem_image2d(b: &mut Bencher) {
                    let d = Uniform::new(<<$pix as Pixel>::Subpixel as Bounded>::min_value() + <<$pix as Pixel>::Subpixel as One>::one(), <<$pix as Pixel>::Subpixel as Bounded>::max_value());
                    let imgbuf1 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                    let imgbuf2 = ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                    let img1: &Image2D<_> = &imgbuf1;
                    let img2: &Image2D<_> = &imgbuf2;
                    b.iter(|| {
                        let _ = (img1 % img2).unwrap();
                    });
                }
            }
        }
    }

    gen_benches!(luma_u8, Luma<u8>);
    gen_benches!(luma_u32, Luma<u32>);
    gen_benches!(luma_f32, Luma<f32>);
}

#[cfg(test)]
mod bench_image {
    use super::*;
    use image::{GenericImage, GrayImage, Luma, ImageBuffer, Pixel, Primitive};

    fn generate_luma(width: u32, height: u32) -> ImageBuffer<Luma<u8>, Vec<u8>>
    {
        let mut rng = thread_rng();
        let mut img = ImageBuffer::new(width, height);
        for pix in img.pixels_mut() {
            *pix = Luma { data: [rng.gen()] }
        }
        img
    }

    #[bench]
    fn image_add_imagebuffer_luma_u8(b: &mut Bencher) {
        let img1 = generate_luma(W, H);
        let img2 = generate_luma(W, H);
        b.iter(|| {
            let mut img = ImageBuffer::new(W, H);
            for ((p1, p2), p) in img1.pixels().zip(img2.pixels()).zip(img.pixels_mut()) {
                *p = Luma { data: [p1[0] + p2[0]] }
            }
        });
    }
}

}
