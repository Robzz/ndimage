//! Contains the definitions of the various traits used in this crate.

use num_traits::{Num, NumRef, RefNum, NumCast, NumOps, Zero};

/// Implemented for primitive pixel types.
pub trait Primitive: Copy + Clone + Num + RefNum<Self> + NumCast + PartialOrd { }

impl<T> Primitive for T
    where T: Copy + Clone + Num + RefNum<T> + NumCast + PartialOrd
{}

/// This trait must be implemented for the types you want to store in an image.
pub trait Pixel: Clone + PartialEq {
    type Subpixel;

    fn n_channels() -> usize;

    fn channels(&self) -> &[Self::Subpixel];

    fn channels_mut(&mut self) -> &mut [Self::Subpixel];

    fn sum<'a>(&'a self) -> Self::Subpixel
        where Self::Subpixel: Primitive
    {
        self.channels().iter().fold(Self::Subpixel::zero(), |s1, s2| s1 + *s2)
    }
}

pub trait PixelOps: Pixel + NumOps { }
pub trait PixelOpsRef: PixelOps + NumRef { }
