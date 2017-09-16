//! Defines a generic 2D image type.

use ndarray;
use ndarray::prelude::*;
use num_traits::{Zero};

use std::iter::{IntoIterator};

use errors::*;
use traits::Pixel;

/// 2-dimensional image type.
pub struct Image2D<P>
    where P: Pixel
{
    buffer: Array2<P>
}

/// Type of immutable `Image2D` views.
pub type Image2DView<'a, P> = ndarray::ArrayView<'a, P, Ix2>;

impl<'a, P> Image2D<P>
    where P: Pixel
{
    /// Create a new image of specified dimensions from a `Vec`.
    ///
    /// **Error**: `InvalidDimensions` if the dimensions do not match the length of `v`.
    pub fn from_vec(w: usize, h: usize, v: Vec<P>) -> Result<Image2D<P>> {
        let buf = try!(Array2::from_shape_vec((w, h), v));
        Ok(Image2D{ buffer: buf })
    }

    /// Return the pixel at the specified coordinates.
    ///
    /// **Panics** if the index is out of bounds.
    pub fn get_pixel(&self, x: usize, y: usize) -> P {
        self.buffer[[x, y]].clone()
    }

    /// Set the pixel at the specified coordinates to the specified value.
    ///
    /// **Panics** if the index is out of bounds.
    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: P) {
        self.buffer[[x, y]] = pixel;
    }

    /// Return the width of the image.
    pub fn width(&self) -> usize { self.buffer.cols() }
    /// Return the height of the image.
    pub fn height(&self) -> usize { self.buffer.rows() }
    /// Return the dimensions of the image as a `(width, height)` tuple.
    pub fn dimensions(&self) -> (usize, usize) { self.buffer.dim() }

    /// Return a view to a subset of the image of specified dimensions starting at the specified
    /// coordinates.
    ///
    /// **Panics** if the specified region crosses image boundaries.
    pub fn sub_image(&'a self, x: usize, y: usize, w: usize, h: usize) -> Image2DView<'a, P> {
        let x = x as isize;
        let y = y as isize;
        let w = w as isize;
        let h = h as isize;
        self.buffer.slice(s![x..x + w, y..y + h])
    }
}

impl<'a, P> IntoIterator for &'a Image2D<P>
    where P: Pixel + 'a
{
    type Item = &'a P;
    type IntoIter = ndarray::iter::Iter<'a, P, Ix2>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a, P> IntoIterator for &'a mut Image2D<P>
    where P: Pixel + 'a
{
    type Item = &'a mut P;
    type IntoIter = ndarray::iter::IterMut<'a, P, Ix2>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter_mut()
    }
}

impl<P> Image2D<P>
    where P: Pixel + Zero
{
    /// Create a new image of specified dimensions filled with zeros.
    pub fn zeros(width: usize, height: usize) -> Image2D<P> {
        Image2D { buffer: Array2::zeros((width, height)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::Luma;

    use std::iter::FromIterator;
    use std::fmt::Debug;

    #[test]
    fn test_from_vec() {
        let v1 = Vec::from_iter((1u8..10u8).map(|n| Luma::new([n])));
        let v2 = Vec::from_iter((1u8..7u8).map(|n| Luma::new([n])));

        assert!(Image2D::from_vec(3, 3, v1.clone()).is_ok());
        assert!(Image2D::from_vec(2, 3, v2.clone()).is_ok());
        assert!(Image2D::from_vec(3, 2, v2.clone()).is_ok());

        assert!(Image2D::from_vec(3, 3, v2.clone()).is_err());
        assert!(Image2D::from_vec(4, 2, v2.clone()).is_err());
    }

    #[test]
    fn test_zeros() {
        fn test_zeros_helper<P>(w: usize, h: usize)
            where P: Pixel + Zero + Debug
        {
            let img = Image2D::<P>::zeros(w, h);
            for pixel in &img {
                assert!(pixel.is_zero());
            }
        };
        test_zeros_helper::<Luma<u8>>(100, 100);
        test_zeros_helper::<Luma<f32>>(100, 100);
    }

    #[test]
    fn test_into_iter() {
        let v: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        for (i, p) in v.into_iter().enumerate() {
            assert!(i + 1 == p);
        }
    }

    #[test]
    fn test_sub_image() {
        let v: Vec<Luma<usize>> = (1..10).map(|n| Luma::new([n])).collect();
        let img = Image2D::from_vec(3, 3, v).unwrap();
        let subimg1 = img.sub_image(1, 1, 2, 2);

        fn subimg_vec_eq<'a>(subimg: Image2DView<'a, Luma<usize>>, v: &Vec<usize>) -> bool {
            let img_iter = subimg.into_iter();
            let v_iter   = v.into_iter();
            match img_iter.len() == v_iter.len() {
                true => !img_iter.zip(v_iter).any(|(p, i)| p.data[0] != *i),
                false => false
            }
        }

        let subimg1_vec: Vec<usize> = vec![5, 6, 8, 9];

        assert!(subimg_vec_eq(subimg1, &subimg1_vec));
    }
}
