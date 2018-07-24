//! # ndimage - A [ndarray](https://crates.io/crates/ndarray) backed image library.
#![deny(missing_docs)]

extern crate byteorder;
#[macro_use]
extern crate failure;
#[macro_use]
pub extern crate ndarray;
extern crate num_traits;
extern crate png;
#[cfg(feature = "rand_integration")]
extern crate rand;
#[cfg(test)]
extern crate tempfile;
extern crate tiff;

pub mod core;
mod helper;
pub mod io;
mod math;
pub mod processing;
