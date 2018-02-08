//! # ndimage - A [ndarray](https://crates.io/crates/ndarray) backed image library.

extern crate byteorder;
#[macro_use] extern crate failure;
#[macro_use] extern crate ndarray;
extern crate num_traits;
extern crate png;

//pub mod dynamic_image;
pub mod image2d;
pub mod io;
pub mod kernel;
pub mod rect;
pub mod traits;
pub mod processing;
mod helper;
mod math;
mod pixel_types;

pub use image2d::Image2D;
pub use traits::{Pixel, PixelOps, Primitive};
pub use pixel_types::{Luma, Rgb};
