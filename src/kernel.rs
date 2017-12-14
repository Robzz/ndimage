use image2d::Image2D;
use rect::Rect;
use traits::{Pixel, Primitive};

use failure::Error;
use std::f64::consts::PI;
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

        Ok(Kernel { elems: elems, radius: radius })
    }

    /// Convolve an image with the kernel. Uses zero-padding for borders.
    pub fn convolve<P, S>(&self, img: &Image2D<P>) -> Image2D<P>
        where P: Pixel<Subpixel=S> + Zero + Add,
              S: Primitive
    {
        let d = 2 * self.radius + 1;
        let n_elems = d * d;
        let n_channels = <P as Pixel>::N_CHANNELS;
        let mut out = img.clone();
        let mut region_accu = Vec::with_capacity((n_elems * n_channels) as usize);
        let mut pix_accu_t = vec![<T as Zero>::zero(); n_channels as usize];
        let mut pix_accu_s = vec![<S as Zero>::zero(); n_channels as usize];
        for ((y, x), dst_pix) in out.enumerate_pixels_mut() {
            let rx = (x as u32).saturating_sub(self.radius);
            let ry = (y as u32).saturating_sub(self.radius);
            let rect = Rect::new(rx, ry, d, d).crop_to_image(img).unwrap();
            for (p, e) in img.rect_iterator(&rect).zip(self.elems.iter()) {
                // Perform the convolution on the kernel floating point type.
                region_accu.extend(p.channels().into_iter().map(|c| *e * <T as NumCast>::from::<S>(*c).unwrap()));
            }
            pix_accu_t.as_mut_slice().into_iter().map(|c| *c = <T as Zero>::zero()).count();
            for convolved_pix in region_accu.as_slice().chunks(n_channels as usize) {
                for i in 0usize..n_channels as usize {
                    pix_accu_t[i] += convolved_pix[i];
                }
            }
            region_accu.clear();
            for i in 0usize..n_channels as usize {
                pix_accu_s[i] = <S as NumCast>::from::<T>(pix_accu_t[i]).unwrap_or(<S as Zero>::zero());
            }
            *dst_pix = P::from_slice(&pix_accu_s);
        }
        out
    }
}

/// Return a gaussian kernel
pub fn gaussian<T>(sigma: T, radius: u32) -> Kernel<T>
    where T: Primitive + Float
{
    let d = 2 * radius + 1;
    let n = d * d;
    let mut v = Vec::with_capacity(n as usize);
    let sigma2 = <f64 as NumCast>::from::<T>(sigma * sigma).unwrap();
    for y in -(radius as i64)..(radius + 1) as i64 {
        for x in -(radius as i64)..(radius + 1) as i64 {
            v.push(<T as NumCast>::from::<f64>(1. / (2. * PI * sigma2) * (-(x*x + y*y) as f64 / (2. * sigma2)).exp()).unwrap());
        }
    }
    Kernel::new(v, radius).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_filter() {
    }
}
