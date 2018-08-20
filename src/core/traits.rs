//! Contains the definitions of the various traits used in this crate.

use num_traits::{clamp, Bounded, NumAssign, NumCast, NumRef, One, Signed, Zero};
#[cfg(feature = "rand_integration")]
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// Implemented for primitive pixel types.
pub trait Primitive:
    Copy + Clone + Debug + Display + Bounded + NumAssign + NumRef + NumCast + PartialOrd + Sync + Send
{
}

impl<T> Primitive for T where
    T: Copy
        + Clone
        + Debug
        + Display
        + Bounded
        + NumAssign
        + NumRef
        + NumCast
        + PartialOrd
        + Sync
        + Send
{}

/// This trait must be implemented for the types you want to store in an image.
pub trait Pixel:
    Clone
    + PartialEq
    + Sync
    + Send
    + Zero
    + One
    + Bounded
    + Add<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + AddAssign
    + for<'a> AddAssign<&'a Self>
    + Sub<Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + SubAssign
    + for<'a> SubAssign<&'a Self>
    + Mul<Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + MulAssign
    + for<'a> MulAssign<&'a Self>
    + Div<Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
    + DivAssign
    + for<'a> DivAssign<&'a Self>
    + Rem<Output = Self>
    + for<'a> Rem<&'a Self, Output = Self>
    + RemAssign
    + for<'a> RemAssign<&'a Self>
{
    /// Type of an individual pixel component.
    // TODO: why is this an associated type again ?
    // If it must be one, can there also be a type parameter and make them equal ?
    type Subpixel: Primitive;

    /// Number of channels contained in the pixel type.
    const N_CHANNELS: u32;

    /// Return a slice containing the different channels of the pixel.
    fn channels(&self) -> &[Self::Subpixel];

    /// Return a mutable slice containing the different channels of the pixel.
    fn channels_mut(&mut self) -> &mut [Self::Subpixel];

    /// Create a new pixel from a slice.
    ///
    /// **Panics**: the length of the slice is not checked, so this function will panic if s.len() is less than the
    /// number of channels in the pixel.
    fn from_slice(s: &[Self::Subpixel]) -> Self;

    /// Create a new pixel where every component is set to the same specified value.
    fn from_value(s: Self::Subpixel) -> Self;

    /// Set the value of the pixel from a slice.
    ///
    /// **Panics**: the length of the slice is not checked, so this function will panic if s.len() is less than the
    /// number of channels in the pixel.
    fn set_to_slice(&mut self, s: &[Self::Subpixel]);

    /// Generate a random value with the Standard distribution.
    #[cfg(feature = "rand_integration")]
    fn rand<R>(rng: &mut R) -> Self
    where
        R: Rng,
        Standard: Distribution<Self::Subpixel>;

    /// Generate a random value with the given distribution.
    #[cfg(feature = "rand_integration")]
    fn rand_with_distr<D, R>(rng: &mut R, distr: &D) -> Self
    where
        R: Rng,
        D: Distribution<Self::Subpixel>;

    /// Compute a new Pixel by applying an operation to each individual pixel component.
    fn map<F>(&self, f: F) -> Self
    where
        F: Fn(Self::Subpixel) -> Self::Subpixel;

    /// Clamp all channels of the pixel between the specified values.
    fn clamp(&mut self, low: Self::Subpixel, high: Self::Subpixel) {
        self.channels_mut()
            .into_iter()
            .map(|c| clamp(*c, low, high))
            .count();
    }

    /// Compute the sum of the pixel components.
    fn sum(&self) -> Self::Subpixel
    where
        Self::Subpixel: Primitive
    {
        self.channels()
            .iter()
            .fold(Self::Subpixel::zero(), |s1, s2| s1 + *s2)
    }

    /// Return a pixel with the absolute value of the given pixel for every channel.
    fn abs(&self) -> Self
    where
        Self::Subpixel: Signed
    {
        self.map(|s| s.abs())
    }
}

/// Trait for types representing image regions.
pub trait Region {
    /// Return `true` if the region contains the specified point, `false` otherwise.
    fn contains(&self, x: u32, y: u32) -> bool;
}

/// Enables casts between pixel types.
///
/// Rust's type system can't (AFAIK) cannot express that both pixel types should have the same number of channels, so
/// this restriction is not enforced. However, all implementations of this trait by pixel within this crate are bounded
/// to only cast between related pixel types only differing by their subpixel associated type. If you're implementing
/// your own pixel types, you should probably do the same.
pub trait PixelCast<O>: Pixel
where
    O: Primitive
{
    /// Result type of the cast.
    type Output: Pixel<Subpixel = O>;

    /// Perform the cast.
    fn cast(&self) -> <Self as PixelCast<O>>::Output;
}
