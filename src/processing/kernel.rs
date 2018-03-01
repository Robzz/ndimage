//! Contains the definitions of the image kernel type and the convolution operation.

use core::{Image2D, Image2DMut, ImageBuffer2D, Rect, Pixel, Primitive};
use helper::generic::f64_to_float;
use math;

use failure::Error;
use num_traits::{NumCast, Zero, Float};

use std::ops::Add;

/// Symmetric odd kernel, whose center is the kernel origin.
#[derive(Debug)]
pub struct Kernel<T> {
    elems: Vec<T>,
    radius: u32
}

impl<T> Kernel<T> where T: Primitive {
    /// Create a new kernel.
    ///
    /// *Error*: if `elems` has an incorrect size, that is `elems.len()` != ((2 * radius) + 1)<sup>2</sup>
    pub fn new(elems: Vec<T>, radius: u32) -> Result<Kernel<T>, Error> {
        let mut s = 2 * radius + 1;
        s *= s;
        ensure!(elems.len() == s as usize, "Vector has an incorrect size: {} (expected {})", elems.len(), s);

        Ok(Kernel { elems, radius })
    }

    /// Convolve an image with the kernel. Uses zero-padding for borders.
    pub fn convolve<P, S>(&self, img: &Image2D<P>) -> ImageBuffer2D<P>
        where P: Pixel<Subpixel=S> + Zero + Add,
              S: Primitive
    {
        let d = 2 * self.radius + 1;
        let n_elems = d * d;
        let n_channels = <P as Pixel>::N_CHANNELS;
        let mut out = img.to_owned();
        let mut region_accu = Vec::with_capacity((n_elems * n_channels) as usize);
        let mut pix_accu_t = vec![<T as Zero>::zero(); n_channels as usize];
        let mut pix_accu_s = vec![<S as Zero>::zero(); n_channels as usize];
        for ((y, x), dst_pix) in out.enumerate_pixels_mut() {
            let rx = (x as u32).saturating_sub(self.radius);
            let ry = (y as u32).saturating_sub(self.radius);
            let rect = Rect::new(rx, ry, d, d).crop_to_image(img).unwrap();
            for (p, e) in img.rect_iter(&rect).zip(self.elems.iter()) {
                // Perform the convolution on the kernel floating point type.
                region_accu.extend(p.channels().into_iter().map(|c| *e * <T as NumCast>::from::<S>(*c).unwrap()));
            }
            pix_accu_t.as_mut_slice().into_iter().map(|c| *c = <T as Zero>::zero()).count();
            for convolved_pix in region_accu.as_slice().chunks(n_channels as usize) {
                for i in 0_usize..n_channels as usize {
                    pix_accu_t[i] += convolved_pix[i];
                }
            }
            region_accu.clear();
            for i in 0_usize..n_channels as usize {
                pix_accu_s[i] = <S as NumCast>::from::<T>(pix_accu_t[i]).unwrap_or_else(<S as Zero>::zero);
            }
            *dst_pix = P::from_slice(&pix_accu_s);
        }
        out
    }
}

impl<T> Kernel<T> where T: Primitive + Float {
/// Return a gaussian kernel
    pub fn gaussian(sigma: T, radius: u32) -> Kernel<T> {
        let d = 2 * radius + 1;
        let n = d * d;
        let mut v = Vec::with_capacity(n as usize);
        let r = <i64 as From<u32>>::from(radius);
        for y in -r..r+1 {
            for x in -r..r+1 {
                v.push(math::gaussian_2d(f64_to_float::<T>(x as f64), f64_to_float::<T>(y as f64), sigma));
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
}

#[cfg(test)]
mod tests {
}
