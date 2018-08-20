//! Histogram type and histogram equalization functions.

use core::{Image2D, ImageBuffer2D, Luma, Pixel};

use num_traits::{NumCast, Zero};

use std::convert::{From, Into};

/// Trait implemented for pixel types for which histogram computation is implemented.
pub trait HistPixel: Pixel + Zero {}
impl HistPixel for Luma<u8> {}
impl HistPixel for Luma<i8> {}

/// Represent a histogram of a greyscale 8-bit image.
pub struct Histogram {
    v: [u32; 256]
}

impl Histogram {
    /// Return the number of pixels in the histogram with the given value, treating the histogram as representing a u8
    /// image.
    pub fn count_u8(&self, val: u8) -> u32 {
        self.v[val as usize]
    }

    /// Return the number of pixels in the histogram with the given value, treating the histogram as representing a i8
    /// image.
    pub fn count_i8(&self, val: u8) -> u32 {
        self.v[val as usize]
    }

    /// Return a reference to the array of histogram bins.
    pub fn bins(&self) -> &[u32; 256] {
        &self.v
    }

    /// Compute the associated cumulative histogram.
    pub fn cumulative(&self) -> Histogram {
        let mut v = [0; 256];
        v[0] = self.v[0];
        for i in 1_usize..256_usize {
            v[i] = v[i - 1] + self.v[i];
        }
        Histogram { v }
    }

    // TODO
    // fn draw(&self) -> Image2D<Rgb<u8>>
}

impl<'a, P> From<&'a Image2D<P>> for Histogram
where
    P: HistPixel
{
    /// Construct a Histogram from an image.
    fn from(img: &'a Image2D<P>) -> Histogram {
        let mut v = [0; 256];
        for pix in img {
            let idx = <u8 as NumCast>::from::<P::Subpixel>(pix.channels()[0]).unwrap();
            v[idx as usize] += 1;
        }
        Histogram { v }
    }
}

/// Adjust the contrast of an image by histogram equalization.
pub fn equalize<P>(img: &Image2D<P>) -> ImageBuffer2D<P>
where
    P: HistPixel
{
    let h: Histogram = img.into();
    let cumul = h.cumulative();
    let m = *cumul.bins().into_iter().max().unwrap();
    let transfer = cumul
        .bins()
        .into_iter()
        .map(|val| ((Into::<f64>::into(*val) * 255.) / (Into::<f64>::into(m))) as u8)
        .collect::<Vec<u8>>();
    let mut equalized = img.to_owned();
    for pix in &mut equalized {
        let idx = <u8 as NumCast>::from::<P::Subpixel>(pix.channels()[0]).unwrap();
        pix.channels_mut()[0] =
            <P::Subpixel as NumCast>::from::<u8>(transfer[idx as usize]).unwrap();
    }
    equalized
}
