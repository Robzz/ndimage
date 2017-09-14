//! Contains the definitions of the various traits used in this crate.

use std::ops::Add;

/// Implemented for primitive pixel types.
pub trait Primitive: Clone + Add { }

impl Primitive for i8 { }
impl Primitive for u8 { }
impl Primitive for i16 { }
impl Primitive for u16 { }
impl Primitive for i32 { }
impl Primitive for u32 { }
impl Primitive for i64 { }
impl Primitive for u64 { }
impl Primitive for usize { }
impl Primitive for isize { }
impl Primitive for f32 { }
impl Primitive for f64 { }

/// This trait must be implemented for the types you want to store in an image.
pub trait Pixel: Clone + Add { }

impl<P> Pixel for P where P: Primitive { }
