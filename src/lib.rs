//! # ndimage - A [ndarray](https://crates.io/crates/ndarray) backed image library.
#![deny(missing_docs)]

extern crate byteorder;
#[macro_use] extern crate failure;
#[macro_use] extern crate ndarray;
extern crate num_traits;
extern crate png;
#[cfg(feature = "rand_integration")] extern crate rand;

pub mod core;
pub mod io;
pub mod processing;
mod helper;
mod math;
