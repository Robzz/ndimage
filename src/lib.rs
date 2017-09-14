//! # ndimage - A [ndarray](https://crates.io/crates/ndarray) backed image library.

#[macro_use] extern crate error_chain;
#[macro_use] extern crate ndarray;
extern crate num_traits;

pub mod errors;
pub mod image2d;
pub mod traits;

pub use image2d::Image2D;
pub use traits::{Pixel, Primitive};
