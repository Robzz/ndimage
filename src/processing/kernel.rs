//! Kernels and image convolution.

use core::{
    cast, padding::*, Image2D, Image2DMut, ImageBuffer2D, Pixel, PixelCast, Primitive, Rect
};
use helper::generic::f64_to_float;
use math;

use failure::Error;
use num_traits::{Float, NumCast};

/// Symmetric odd kernel, whose center is the kernel origin.
// TODO: make iterable, indexable, etc...
// TODO: should this be constrained to float types ?
// TODO: store as an image2D instead ?
#[derive(Debug)]
pub struct Kernel<T>
where
    T: Primitive + Float
{
    elems: Vec<T>,
    radius: u32
}

impl<T> Kernel<T>
where
    T: Primitive + Float
{
    /// Create a new kernel.
    ///
    /// *Error*: if `elems` has an incorrect size, that is `elems.len()` != ((2 * radius) + 1)<sup>2</sup>
    pub fn new(elems: Vec<T>, radius: u32) -> Result<Kernel<T>, Error> {
        let mut s = 2 * radius + 1;
        s *= s;
        ensure!(
            elems.len() == s as usize,
            "Vector has an incorrect size: {} (expected {})",
            elems.len(),
            s
        );

        Ok(Kernel { elems, radius })
    }

    /// Convolve an image with the kernel, padding the image by the specified method to handle
    /// boundary conditions. The convolution is internally performed by casting the input image
    /// into the kernel primitive type. The convolution result is cast into the `O` type parameter
    /// before returning.
    pub fn convolve<Ps, Pt, S, O>(
        &self,
        img: &Image2D<Ps>,
        padding: Padding
    ) -> ImageBuffer2D<<Pt as PixelCast<O>>::Output>
    where
        Ps: Pixel<Subpixel = S> + PixelCast<T, Output = Pt>,
        Pt: Pixel<Subpixel = T> + PixelCast<O>,
        S: Primitive,
        O: Primitive
    {
        let padded = cast::<T, Ps>(&padding.apply(img, self.radius));
        let d = 2 * self.radius + 1;
        let n_elems = d * d;
        let mut out = ImageBuffer2D::new(img.width(), img.height());
        let mut region_accu = Vec::<<Ps as PixelCast<T>>::Output>::with_capacity(n_elems as usize);
        let mut pix_accu;
        for ((y, x), dst_pix) in out.enumerate_pixels_mut() {
            let rx = x as u32;
            let ry = y as u32;
            let rect = Rect::new(rx, ry, d, d);
            for (p, e) in padded.rect_iter(rect).zip(self.elems.iter()) {
                region_accu.push(<Ps as PixelCast<T>>::Output::from_value(*e) * p);
            }
            pix_accu = <Ps as PixelCast<T>>::Output::zero();
            for convolved_pix in &region_accu {
                pix_accu += convolved_pix;
            }
            region_accu.clear();
            let max = <T as NumCast>::from(S::max_value()).unwrap();
            let min = <T as NumCast>::from(S::min_value()).unwrap();
            pix_accu.clamp(min, max);
            *dst_pix = pix_accu.cast();
        }
        out
    }
}

impl<T> Kernel<T>
where
    T: Primitive + Float
{
    /// Return a gaussian kernel
    pub fn gaussian(sigma: T, radius: u32) -> Kernel<T> {
        let d = 2 * radius + 1;
        let n = d * d;
        let mut v = Vec::with_capacity(n as usize);
        let r = <i64 as From<u32>>::from(radius);
        for y in -r..r + 1 {
            for x in -r..r + 1 {
                v.push(math::gaussian_2d(
                    f64_to_float::<T>(x as f64),
                    f64_to_float::<T>(y as f64),
                    sigma
                ));
            }
        }
        Kernel::new(v, radius).unwrap()
    }

    /// Return a box kernel
    pub fn box_(radius: u32) -> Kernel<T> {
        let d = 2 * radius + 1;
        let n = d * d;
        let v = vec![f64_to_float::<T>(1. / <f64 as From<u32>>::from(n)); n as usize];
        Kernel::new(v, radius).unwrap()
    }

    /// Return a 3x3 Sobel kernel to compute the x derivative in the positive direction.
    pub fn sobel_x_3x3() -> Kernel<T> {
        let zero = T::zero();
        let one = <T as NumCast>::from(1).unwrap();
        let two = <T as NumCast>::from(2).unwrap();
        Kernel::new(vec![-one, zero, one, -two, zero, two, -one, zero, one], 1).unwrap()
    }

    /// Return a 3x3 Sobel kernel to compute the y derivative in the positive direction.
    pub fn sobel_y_3x3() -> Kernel<T> {
        let zero = T::zero();
        let one = <T as NumCast>::from(1).unwrap();
        let two = <T as NumCast>::from(2).unwrap();
        Kernel::new(vec![-one, -two, -one, zero, zero, zero, one, two, one], 1).unwrap()
    }
}

#[cfg(test)]
mod tests {}
