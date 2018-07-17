//! Colorspace conversion routines.

use core::{Pixel, Primitive, Image2D, ImageBuffer2D, Luma as PLuma, LumaA, Rgb, RgbA};

use num_traits::NumCast;

use std::marker::PhantomData;

/// Implemented by types describing a colorspace.
// TODO custom derive ?
pub trait Colorspace {
    /// Pixel type in which values are represented.
    type Pixel: Pixel;
}

/// Enables conversions between colorspaces.
pub trait FromColor<C, P>
where
    C: Colorspace,
    P: Pixel
{
    /// Perform the conversion for a single pixel.
    fn from_pixel(&self, from: &C, pix: &C::Pixel) -> P;

    /// Perform the conversion for a whole image.
    fn from_image(&self, from: &C, img: &Image2D<C::Pixel>) -> ImageBuffer2D<P> {
        let converted_vec: Vec<P> = img.into_iter().map(|p| self.from_pixel(from, p)).collect();
        ImageBuffer2D::from_vec(img.width(), img.height(), converted_vec).unwrap()
    }
}

/// Linear colorspace.
pub struct Linear<P> where P: Pixel {
    _phantom: PhantomData<P>
}

impl<P> Colorspace for Linear<P>
    where P: Pixel
{
    type Pixel = P;
}

/// Gamma colorspace.
pub struct Gamma<P> where P: Pixel {
    /// Gamma value.
    pub gamma: f64,
    _phantom: PhantomData<P>
}

impl<P> Colorspace for Gamma<P>
    where P: Pixel
{
    type Pixel = P;
}

/// Single channel representing luminance.
pub struct Luminance<S> where S: Primitive
{
    _phantom: PhantomData<S>
}

impl<S> Colorspace for Luminance<S> where S: Primitive
{
    type Pixel = PLuma<S>;
}

/// Single channel representing luma.
pub struct Luma<S> where S: Primitive
{
    _phantom: PhantomData<S>
}

impl<S> Colorspace for Luma<S> where S: Primitive
{
    type Pixel = PLuma<S>;
}

impl<S> FromColor<Linear<Rgb<S>>, PLuma<S>> for Luma<S>
    where S: Primitive
{
    fn from_pixel(&self, _: &Linear<Rgb<S>>, pix: &Rgb<S>) -> PLuma<S> {
        let r_f64 = <f64 as NumCast>::from(pix[0]).unwrap();
        let g_f64 = <f64 as NumCast>::from(pix[1]).unwrap();
        let b_f64 = <f64 as NumCast>::from(pix[2]).unwrap();
        PLuma::new([<S as NumCast>::from(0.2126 * r_f64 + 0.7152 * g_f64 + 0.0722 * b_f64).unwrap()])
    }
}

impl<P> FromColor<Gamma<P>, P> for Linear<P>
    where P: Pixel
{
    fn from_pixel(&self, from: &Gamma<P>, pix: &P) -> P {
        pix.map(|v| {
            let v_f64 = <f64 as NumCast>::from(v).unwrap();
            <P::Subpixel as NumCast>::from(v_f64.powf(1. / from.gamma)).unwrap()
        })
    }
}
