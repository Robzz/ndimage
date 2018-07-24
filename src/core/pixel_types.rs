//! Contains the definitions of the various pixel types defined in this crate.

use num_traits::cast::cast;
use num_traits::{Bounded, One, Zero};
#[cfg(feature = "rand_integration")]
use rand::{
    distributions::{Distribution, Standard}, Rng,
};

use core::{Pixel, PixelCast, Primitive};

use std::convert::From;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

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
    Other,
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

// TODO: impl_op! macro

macro_rules! impl_pixel_op {
    ($pix_t:ident : $n_channels:expr, $op:ident, $op_fn:ident) => {
        impl<P> $op for $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: $pix_t<P>) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = s.$op_fn(r);
                }
                $pix_t { data }
            }
        }

        impl<'a, P> $op<$pix_t<P>> for &'a $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: $pix_t<P>) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = s.$op_fn(r);
                }
                $pix_t { data }
            }
        }

        impl<'a, P> $op<&'a $pix_t<P>> for $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: &'a $pix_t<P>) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = s.$op_fn(r);
                }
                $pix_t { data }
            }
        }

        impl<'a, 'b, P> $op<&'a $pix_t<P>> for &'b $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: &'a $pix_t<P>) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for ((n, s), r) in data.iter_mut().zip(self.data.iter()).zip(rhs.data.iter()) {
                    *n = s.$op_fn(r);
                }
                $pix_t { data }
            }
        }

        impl<P> $op<P> for $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: P) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for (n, s) in data.iter_mut().zip(self.data.iter()) {
                    *n = s.$op_fn(rhs);
                }
                $pix_t { data }
            }
        }

        impl<'a, P> $op<P> for &'a $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: P) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for (n, s) in data.iter_mut().zip(self.data.iter()) {
                    *n = s.$op_fn(rhs);
                }
                $pix_t { data }
            }
        }

        impl<'a, P> $op<&'a P> for $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: &'a P) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for (n, s) in data.iter_mut().zip(self.data.iter()) {
                    *n = s.$op_fn(rhs);
                }
                $pix_t { data }
            }
        }

        impl<'a, 'b, P> $op<&'a P> for &'b $pix_t<P>
        where
            P: Primitive,
        {
            type Output = $pix_t<P>;

            fn $op_fn(self, rhs: &'a P) -> $pix_t<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for (n, s) in data.iter_mut().zip(self.data.iter()) {
                    *n = s.$op_fn(rhs);
                }
                $pix_t { data }
            }
        }
    };
}

macro_rules! impl_pixel_op_assign {
    ($pix_t:ident : $n_channels:expr, $op:ident, $op_fn:ident) => {
        impl<P> $op for $pix_t<P>
        where
            P: Primitive,
        {
            fn $op_fn(&mut self, rhs: $pix_t<P>) {
                for (s, r) in self.data.iter_mut().zip(rhs.data.iter()) {
                    s.$op_fn(*r);
                }
            }
        }

        impl<'a, P> $op<&'a $pix_t<P>> for $pix_t<P>
        where
            P: Primitive,
        {
            fn $op_fn(&mut self, rhs: &'a $pix_t<P>) {
                for (s, r) in self.data.iter_mut().zip(rhs.data.iter()) {
                    s.$op_fn(*r);
                }
            }
        }

        impl<P> $op<P> for $pix_t<P>
        where
            P: Primitive,
        {
            fn $op_fn(&mut self, rhs: P) {
                for s in &mut self.data {
                    s.$op_fn(rhs);
                }
            }
        }

        impl<'a, P> $op<&'a P> for $pix_t<P>
        where
            P: Primitive,
        {
            fn $op_fn(&mut self, rhs: &'a P) {
                for s in &mut self.data {
                    s.$op_fn(*rhs);
                }
            }
        }
    };
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

        impl_pixel_op!($name: $n_channels, Add, add);
        impl_pixel_op!($name: $n_channels, Sub, sub);
        impl_pixel_op!($name: $n_channels, Mul, mul);
        impl_pixel_op!($name: $n_channels, Div, div);
        impl_pixel_op!($name: $n_channels, Rem, rem);
        impl_pixel_op_assign!($name: $n_channels, AddAssign, add_assign);
        impl_pixel_op_assign!($name: $n_channels, SubAssign, sub_assign);
        impl_pixel_op_assign!($name: $n_channels, MulAssign, mul_assign);
        impl_pixel_op_assign!($name: $n_channels, DivAssign, div_assign);
        impl_pixel_op_assign!($name: $n_channels, RemAssign, rem_assign);

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
                for c in data.iter_mut().take($n_channels) {
                    *c = rng.sample(distr);
                }
                Self { data }
            }
        }

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

#[cfg(test)]
mod tests {
    use core::Luma;

    #[test]
    fn test_pixel_add() {
        let l1 = Luma::new([5u8]);
        let l2 = Luma::new([7u8]);
        let l3 = Luma::new([12u8]);
        assert_eq!(&l1 + &l2, l3.clone());
        assert_eq!(&l1 + l2.clone(), l3.clone());
        assert_eq!(l1.clone() + &l2, l3.clone());
        assert_eq!(l1.clone() + l2.clone(), l3.clone());

        let l4 = Luma::new([17u8]);
        assert_eq!(l3 + 5u8, l4.clone());
        assert_eq!(l3 + &5u8, l4.clone());
        assert_eq!(&l3 + 5u8, l4.clone());
        assert_eq!(&l3 + &5u8, l4.clone());
    }

    #[test]
    fn test_pixel_sub() {
        let l1 = Luma::new([15u8]);
        let l2 = Luma::new([7u8]);
        let l3 = Luma::new([8u8]);
        assert_eq!(&l1 - &l2, l3.clone());
        assert_eq!(&l1 - l2.clone(), l3.clone());
        assert_eq!(l1.clone() - &l2, l3.clone());
        assert_eq!(l1.clone() - l2.clone(), l3.clone());

        let l4 = Luma::new([3u8]);
        assert_eq!(l3 - 5u8, l4.clone());
        assert_eq!(l3 - &5u8, l4.clone());
        assert_eq!(&l3 - 5u8, l4.clone());
        assert_eq!(&l3 - &5u8, l4.clone());
    }
}
