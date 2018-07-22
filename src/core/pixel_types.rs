//! Contains the definitions of the various pixel types defined in this crate.

use num_traits::cast::cast;
use num_traits::{Bounded, One, Zero};
#[cfg(feature = "rand_integration")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use core::{Pixel, PixelCast, PixelOps, Primitive};

use std::convert::From;
use std::ops::{Add, Div, Index, IndexMut, Mul, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerate the supported subpixel types.
pub enum SubpixelType {
    /// u8
    U8,
    /// u16
    U16,
    /// u32
    U32,
    /// u64
    U64,
    /// u8
    I8,
    /// u16
    I16,
    /// u32
    I32,
    /// u64
    I64,
    /// u32
    F32,
    /// u64
    F64,
    /// Intended for custom subpixel types.
    Other
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerate the supported pixel types.
pub enum PixelType {
    /// Single channel, i.e. grayscale.
    Luma,
    /// Dual channel, i.e. grayscale with alpha.
    LumaA,
    /// Triple channel, i.e. color.
    Rgb,
    /// Quad channel, i.e. color with alpha.
    RgbA,
}

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

        impl<'a, P> Add<$name<P>> for &'a $name<P>
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

        impl<'a, P> Add<&'a $name<P>> for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn add(self, rhs: &'a $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s + *r;
                }
                $name { data }
            }
        }

        impl<'a, 'b, P> Add<&'a $name<P>> for &'b $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn add(self, rhs: &'a $name<P>) -> $name<P> {
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

        impl<'a, P> Sub<$name<P>> for &'a $name<P>
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

        impl<'a, P> Sub<&'a $name<P>> for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn sub(self, rhs: &'a $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s - *r;
                }
                $name { data }
            }
        }

        impl<'a, 'b, P> Sub<&'a $name<P>> for &'b $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn sub(self, rhs: &'a $name<P>) -> $name<P> {
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

        impl<'a, P> Mul<$name<P>> for &'a $name<P>
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

        impl<'a, P> Mul<&'a $name<P>> for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn mul(self, rhs: &'a $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s * *r;
                }
                $name { data }
            }
        }

        impl<'a, 'b, P> Mul<&'a $name<P>> for &'b $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn mul(self, rhs: &'a $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s * *r;
                }
                $name { data }
            }
        }

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

        impl<'a, P> Div<$name<P>> for &'a $name<P>
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

        impl<'a, P> Div<&'a $name<P>> for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn div(self, rhs: &'a $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s / *r;
                }
                $name { data }
            }
        }

        impl<'a, 'b, P> Div<&'a $name<P>> for &'b $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn div(self, rhs: &'a $name<P>) -> $name<P> {
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

        impl<'a, P> Rem<$name<P>> for &'a $name<P>
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

        impl<'a, P> Rem<&'a $name<P>> for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn rem(self, rhs: &'a $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = *s % *r;
                }
                $name { data }
            }
        }

        impl<'a, 'b, P> Rem<&'a $name<P>> for &'b $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn rem(self, rhs: &'a $name<P>) -> $name<P> {
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

        impl<P> Bounded for $name<P>
            where P: Primitive
        {
            fn min_value() -> $name<P> {
                $name { data: [<P as Bounded>::min_value(); $n_channels ] }
            }

            fn max_value() -> $name<P> {
                $name { data: [<P as Bounded>::max_value(); $n_channels ] }
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

        impl<P> IndexMut<u8> for $name<P>
            where P: Primitive
        {
            fn index_mut(&mut self, index: u8) -> &mut P {
                &mut self.data[index as usize]
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

            #[cfg(feature = "rand_integration")]
            fn rand<R>(rng: &mut R) -> $name<P>
                where R: Rng,
                      Standard: Distribution<[P; $n_channels]>,

            {
                Self { data: rng.gen() }
            }

            #[cfg(feature = "rand_integration")]
            fn rand_with_distr<D, R>(rng: &mut R, distr: &D) -> $name<P>
                where R: Rng,
                      D: Distribution<P>
            {
                let mut data = [P::zero(); $n_channels];
                for i in 0..$n_channels {
                    data[i] = rng.sample(distr);
                }
                Self { data }
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
where
    P: Primitive,
{
    fn from(pixel: LumaA<P>) -> Luma<P> {
        Luma {
            data: [pixel.data[0]],
        }
    }
}

impl<'a, P> From<&'a LumaA<P>> for Luma<P>
where
    P: Primitive,
{
    fn from(pixel: &'a LumaA<P>) -> Luma<P> {
        Luma {
            data: [pixel.data[0]],
        }
    }
}

impl<P> From<RgbA<P>> for Rgb<P>
where
    P: Primitive,
{
    fn from(pixel: RgbA<P>) -> Rgb<P> {
        Rgb {
            data: [pixel.data[0], pixel.data[1], pixel.data[2]],
        }
    }
}

impl<'a, P> From<&'a RgbA<P>> for Rgb<P>
where
    P: Primitive,
{
    fn from(pixel: &'a RgbA<P>) -> Rgb<P> {
        Rgb {
            data: [pixel.data[0], pixel.data[1], pixel.data[2]],
        }
    }
}

impl<P> From<P> for Luma<P>
where
    P: Primitive,
{
    fn from(data: P) -> Luma<P> {
        Luma { data: [data] }
    }
}
