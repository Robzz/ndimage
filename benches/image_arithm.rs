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
    use rand::{
        distributions::{uniform::SampleUniform, Distribution, Standard, Uniform},
        thread_rng
    };
    use test::Bencher;

    #[cfg(test)]
    mod bench_ndimage {
        use super::*;
        use ndimage::core::{Image2D, ImageBuffer2D, Luma, Pixel};

        macro_rules! gen_benches {
            ($name:ident, $pix:ty) => {
                mod $name {
                    use super::*;

                    #[bench]
                    fn add_imagebuffer2d(b: &mut Bencher) {
                        let img1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        let img2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        b.iter(|| {
                            let _ = (&img1 + &img2).unwrap();
                        });
                    }

                    #[bench]
                    fn add_image2d(b: &mut Bencher) {
                        let imgbuf1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        let imgbuf2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());

                        let img1: &Image2D<_> = &imgbuf1;
                        let img2: &Image2D<_> = &imgbuf2;
                        b.iter(|| {
                            let _ = (img1 + img2).unwrap();
                        });
                    }
                    #[bench]
                    fn sub_imagebuffer2d(b: &mut Bencher) {
                        let img1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        let img2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        b.iter(|| {
                            let _ = (&img1 - &img2).unwrap();
                        });
                    }

                    #[bench]
                    fn sub_image2d(b: &mut Bencher) {
                        let imgbuf1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        let imgbuf2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());

                        let img1: &Image2D<_> = &imgbuf1;
                        let img2: &Image2D<_> = &imgbuf2;
                        b.iter(|| {
                            let _ = (img1 - img2).unwrap();
                        });
                    }
                    #[bench]
                    fn mul_imagebuffer2d(b: &mut Bencher) {
                        let img1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        let img2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        b.iter(|| {
                            let _ = (&img1 * &img2).unwrap();
                        });
                    }

                    #[bench]
                    fn mul_image2d(b: &mut Bencher) {
                        let imgbuf1 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());
                        let imgbuf2 = ImageBuffer2D::<$pix>::rand(W, H, &mut thread_rng());

                        let img1: &Image2D<_> = &imgbuf1;
                        let img2: &Image2D<_> = &imgbuf2;
                        b.iter(|| {
                            let _ = (img1 * img2).unwrap();
                        });
                    }
                    #[bench]
                    fn div_imagebuffer2d(b: &mut Bencher) {
                        let d = Uniform::new(
                            <<$pix as Pixel>::Subpixel as Bounded>::min_value()
                                + <<$pix as Pixel>::Subpixel as One>::one(),
                            <<$pix as Pixel>::Subpixel as Bounded>::max_value()
                        );
                        let img1 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                        let img2 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                        b.iter(|| {
                            let _ = (&img1 / &img2).unwrap();
                        });
                    }

                    #[bench]
                    fn div_image2d(b: &mut Bencher) {
                        let d = Uniform::new(
                            <<$pix as Pixel>::Subpixel as Bounded>::min_value()
                                + <<$pix as Pixel>::Subpixel as One>::one(),
                            <<$pix as Pixel>::Subpixel as Bounded>::max_value()
                        );
                        let imgbuf1 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                        let imgbuf2 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                        let img1: &Image2D<_> = &imgbuf1;
                        let img2: &Image2D<_> = &imgbuf2;
                        b.iter(|| {
                            let _ = (img1 / img2).unwrap();
                        });
                    }

                    #[bench]
                    fn rem_imagebuffer2d(b: &mut Bencher) {
                        let d = Uniform::new(
                            <<$pix as Pixel>::Subpixel as Bounded>::min_value()
                                + <<$pix as Pixel>::Subpixel as One>::one(),
                            <<$pix as Pixel>::Subpixel as Bounded>::max_value()
                        );
                        let img1 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                        let img2 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                        b.iter(|| {
                            let _ = (&img1 % &img2).unwrap();
                        });
                    }

                    #[bench]
                    fn rem_image2d(b: &mut Bencher) {
                        let d = Uniform::new(
                            <<$pix as Pixel>::Subpixel as Bounded>::min_value()
                                + <<$pix as Pixel>::Subpixel as One>::one(),
                            <<$pix as Pixel>::Subpixel as Bounded>::max_value()
                        );
                        let imgbuf1 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);
                        let imgbuf2 =
                            ImageBuffer2D::<$pix>::rand_with_distr(W, H, &mut thread_rng(), &d);

                        let img1: &Image2D<_> = &imgbuf1;
                        let img2: &Image2D<_> = &imgbuf2;
                        b.iter(|| {
                            let _ = (img1 % img2).unwrap();
                        });
                    }
                }
            };
        }

        gen_benches!(luma_u8, Luma<u8>);
        gen_benches!(luma_u32, Luma<u32>);
        gen_benches!(luma_f32, Luma<f32>);
    }

    #[cfg(test)]
    mod bench_image {
        use super::*;
        use image::{ImageBuffer, Luma, Primitive};

        use rand::Rng;

        fn generate_luma<S>(width: u32, height: u32) -> ImageBuffer<Luma<S>, Vec<S>>
        where
            S: Primitive + 'static,
            Standard: Distribution<S>
        {
            let mut rng = thread_rng();
            let mut img = ImageBuffer::new(width, height);
            for pix in img.pixels_mut() {
                *pix = Luma { data: [rng.gen()] }
            }
            img
        }

        fn generate_luma_nonzero<S>(width: u32, height: u32) -> ImageBuffer<Luma<S>, Vec<S>>
        where
            S: Primitive + SampleUniform + Bounded + 'static,
            Standard: Distribution<S>
        {
            let distr = Uniform::new(<S as One>::one(), <S as Bounded>::max_value());
            let mut rng = thread_rng();
            let mut img = ImageBuffer::new(width, height);
            for pix in img.pixels_mut() {
                *pix = Luma {
                    data: [rng.sample(&distr)]
                }
            }
            img
        }

        macro_rules! gen_benches_luma {
            ($name:ident, $subpix:ty) => {
                mod $name {
                    use super::*;

                    #[bench]
                    fn add_imagebuffer2d(b: &mut Bencher) {
                        let img1 = generate_luma::<$subpix>(W, H);
                        let img2 = generate_luma::<$subpix>(W, H);

                        b.iter(|| {
                            let mut img = ImageBuffer::new(W, H);
                            for ((p1, p2), p) in
                                img1.pixels().zip(img2.pixels()).zip(img.pixels_mut())
                            {
                                *p = Luma {
                                    data: [p1[0] + p2[0]]
                                };
                            }
                        });
                    }

                    #[bench]
                    fn sub_image2d(b: &mut Bencher) {
                        let img1 = generate_luma::<$subpix>(W, H);
                        let img2 = generate_luma::<$subpix>(W, H);

                        b.iter(|| {
                            let mut img = ImageBuffer::new(W, H);
                            for ((p1, p2), p) in
                                img1.pixels().zip(img2.pixels()).zip(img.pixels_mut())
                            {
                                *p = Luma {
                                    data: [p1[0] - p2[0]]
                                }
                            }
                        });
                    }

                    #[bench]
                    fn mul_imagebuffer2d(b: &mut Bencher) {
                        let img1 = generate_luma::<$subpix>(W, H);
                        let img2 = generate_luma::<$subpix>(W, H);

                        b.iter(|| {
                            let mut img = ImageBuffer::new(W, H);
                            for ((p1, p2), p) in
                                img1.pixels().zip(img2.pixels()).zip(img.pixels_mut())
                            {
                                *p = Luma {
                                    data: [p1[0] * p2[0]]
                                }
                            }
                        });
                    }

                    #[bench]
                    fn div_imagebuffer2d(b: &mut Bencher) {
                        let img1 = generate_luma_nonzero::<$subpix>(W, H);
                        let img2 = generate_luma_nonzero::<$subpix>(W, H);

                        b.iter(|| {
                            let mut img = ImageBuffer::new(W, H);
                            for ((p1, p2), p) in
                                img1.pixels().zip(img2.pixels()).zip(img.pixels_mut())
                            {
                                *p = Luma {
                                    data: [p1[0] / p2[0]]
                                }
                            }
                        });
                    }

                    #[bench]
                    fn rem_imagebuffer2d(b: &mut Bencher) {
                        let img1 = generate_luma_nonzero::<$subpix>(W, H);
                        let img2 = generate_luma_nonzero::<$subpix>(W, H);

                        b.iter(|| {
                            let mut img = ImageBuffer::new(W, H);
                            for ((p1, p2), p) in
                                img1.pixels().zip(img2.pixels()).zip(img.pixels_mut())
                            {
                                *p = Luma {
                                    data: [p1[0] % p2[0]]
                                }
                            }
                        });
                    }
                }
            };
        }

        gen_benches_luma!(luma_u8, u8);
        gen_benches_luma!(luma_u32, u32);
        gen_benches_luma!(luma_f32, f32);
    }

}
