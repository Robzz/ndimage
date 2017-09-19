//! # ndimage - A [ndarray](https://crates.io/crates/ndarray) backed image library.

#[macro_use] extern crate error_chain;
#[macro_use] extern crate ndarray;
extern crate num_traits;

pub mod errors;
pub mod image2d;
pub mod rect;
pub mod traits;
mod pixel_types;

pub use image2d::Image2D;
pub use traits::{Pixel, PixelOps, Primitive};
pub use pixel_types::{Luma, Rgb};
