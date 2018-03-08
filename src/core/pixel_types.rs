//! Contains the definitions of the various pixel types defined in this crate.

use num_traits::{Zero, One};
use num_traits::cast::cast;
#[cfg(feature="rand_integration")] use rand::{Rand, Rng};

use core::{Primitive, Pixel, PixelOps, PixelCast};

use std::convert::From;
use std::ops::{Add, Sub, Mul, Div, Rem, Index};

macro_rules! impl_pixels {
    ( $( $(#[$attr:meta])* $name:ident, $n_channels:expr);+ ) =>
    {$(
        #[derive(Debug, Copy, Clone, PartialEq)]
        $( #[$attr] )*
        pub struct $name<P>
            where P: Primitive
        {
            /// Pixel channels
            pub data: [P; $n_channels]
        }

        impl<P> $name<P>
            where P: Primitive
        {
            /// Construct a pixel from an array representing its' channels.
            pub fn new(data: [P; $n_channels]) -> $name<P> {
                $name { data }
            }
        }

        impl<P> Add for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn add(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s + *r;
                }
                $name { data }
            }
        }

        impl<P> Sub for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn sub(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s - *r;
                }
                $name { data }
            }
        }

        impl<'a, 'b, P> Sub<&'b $name<P>> for &'a $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn sub(self, rhs: &'b $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s - *r;
                }
                $name { data }
            }
        }

        impl<P> Mul for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn mul(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s * *r;
                }
                $name { data }
            }
        }

        //impl<'a, P> Mul for &'a $name<P>
            //where P: Primitive
        //{
            //type Output = $name<P>;

            //fn mul(&'a self, &'a rhs: $name<P>) -> $name<P> {
                //let mut data = [<P as Zero>::zero(); $n_channels];
                //for i in 0..$n_channels {
                    //data[i] = self.data[i] * rhs.data[i];
                //}
                //$name { data: data }
            //}
        //}

        impl<P> Div for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn div(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s / *r;
                }
                $name { data }
            }
        }

        impl<P> Rem for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn rem(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s % *r;
                }
                $name { data }
            }
        }

        impl<P> Zero for $name<P>
            where P: Primitive
        {
            fn zero() -> $name<P> {
                $name { data: [<P as Zero>::zero(); $n_channels] }
            }

            fn is_zero(&self) -> bool {
                self.data.iter().map(|p| p.is_zero()).fold(true, |acc, b| b && acc)
            }
        }

        impl<P> One for $name<P>
            where P: Primitive
        {
            fn one() -> $name<P> {
                $name { data: [<P as One>::one(); $n_channels ] }
            }
        }

        impl<P> From<[P; $n_channels]> for $name<P>
            where P: Primitive
        {
            fn from(array: [P; $n_channels]) -> $name<P> {
                $name { data: array }
            }
        }

        impl<P> Index<u8> for $name<P>
            where P: Primitive
        {
            type Output = P;

            fn index(&self, index: u8) -> &P {
                &self.data[index as usize]
            }
        }

        impl<P> Pixel for $name<P>
            where P: Primitive
        {
            type Subpixel = P;

            const N_CHANNELS: u32 = $n_channels;

            fn channels(&self) -> &[P] { &self.data }

            fn channels_mut(&mut self) -> &mut [P] { &mut self.data }

            fn from_slice(s: &[Self::Subpixel]) -> $name<P> {
                let mut p = $name::zero();
                for (n, e) in p.data.iter_mut().zip(s.iter()) {
                    *n = *e;
                }
                p
            }

            fn set_to_slice(&mut self, s: &[Self::Subpixel]) {
                for (n, e) in self.data.iter_mut().zip(s.iter()) {
                    *n = *e;
                }
            }

            fn map<F>(&self, f: F) -> Self
                where F: Fn(Self::Subpixel) -> Self::Subpixel
            {
                let mut p = <Self as Zero>::zero();
                for (dst, src) in p.channels_mut().into_iter().zip(self.data.into_iter()) {
                    *dst = f(*src);
                }
                p
            }
        }

        impl<P> PixelOps for $name<P>
            where P: Primitive
        { }

        impl<S, O> PixelCast<$name<O>, S, O> for $name<S>
            where O: Primitive,
                  S: Primitive
        {
            fn cast_from(&mut self, other: &$name<O>) {
                for (src, dst) in other.channels().into_iter().zip(self.channels_mut().into_iter()) {
                    *dst = cast::<O, S>(src.clone()).unwrap_or(<S as Zero>::zero());
                }
            }

            fn cast_to(&self, other: &mut $name<O>) {
                for (dst, src) in other.channels_mut().into_iter().zip(self.channels().into_iter()) {
                    *dst = cast::<S, O>(src.clone()).unwrap_or(<O as Zero>::zero());
                }
            }
        }

        #[cfg(feature = "rand_integration")]
        impl<P> Rand for $name<P>
            where P: Primitive
        {
            fn rand<R>(rng: &mut R) -> $name<P>
                where R: Rng
            {
                let mut p = [P::zero(); $n_channels];
                for c in p.iter_mut().take($n_channels) {
                   *c = P::rand(rng);
                }

                $name::new(p)
            }
        }
    )+}
}

impl_pixels!(
    /// Grayscale pixel type
    Luma, 1;
    /// Grayscale with alpha pixel type
    LumaA, 2;
    /// RGB pixel type
    Rgb, 3;
    /// RGB with alpha pixel type
    RgbA, 4
);

impl<P> From<LumaA<P>> for Luma<P>
    where P: Primitive
{
    fn from(pixel: LumaA<P>) -> Luma<P> {
        Luma { data: [pixel.data[0]] }
    }
}

impl<'a, P> From<&'a LumaA<P>> for Luma<P>
    where P: Primitive
{
    fn from(pixel: &'a LumaA<P>) -> Luma<P> {
        Luma { data: [pixel.data[0]] }
    }
}

impl<P> From<RgbA<P>> for Rgb<P>
    where P: Primitive
{
    fn from(pixel: RgbA<P>) -> Rgb<P> {
        Rgb { data: [pixel.data[0], pixel.data[1], pixel.data[2]] }
    }
}

impl<'a, P> From<&'a RgbA<P>> for Rgb<P>
    where P: Primitive
{
    fn from(pixel: &'a RgbA<P>) -> Rgb<P> {
        Rgb { data: [pixel.data[0], pixel.data[1], pixel.data[2]] }
    }
}

impl<P> From<P> for Luma<P>
    where P: Primitive
{
    fn from(data: P) -> Luma<P> {
        Luma { data: [data] }
    }
}
