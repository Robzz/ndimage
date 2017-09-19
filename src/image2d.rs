//! Defines a generic 2D image type.

use ndarray;
use ndarray::prelude::*;
use num_traits::{Zero};

use std::cmp::min;
use std::iter::{IntoIterator};

use errors::*;
use rect::Rect;
use traits::Pixel;

/// 2-dimensional image type.
pub struct Image2D<P>
    where P: Pixel
{
    buffer: Array2<P>
}

/// Type of immutable `Image2D` views.
pub type Image2DView<'a, P> = ndarray::ArrayView<'a, P, Ix2>;
pub type Image2DViewMut<'a, P> = ndarray::ArrayViewMut<'a, P, Ix2>;
pub type Iter<'a, P> = ndarray::iter::Iter<'a, P, Ix2>;
pub type IterMut<'a, P> = ndarray::iter::IterMut<'a, P, Ix2>;

impl<'a, P> Image2D<P>
    where P: Pixel
{
    /// Create a new image of specified dimensions filled with zeros.
    pub fn new(width: u32, height: u32) -> Image2D<P>
        where P: Pixel + Zero
    {
        Image2D { buffer: Array2::zeros((height as usize, width as usize)) }
    }

    /// Create a new image of specified dimensions from a `Vec` of the specified pixel type.
    ///
    /// **Error**: `InvalidDimensions` if the dimensions do not match the length of `v`.
    pub fn from_vec(w: u32, h: u32, v: Vec<P>) -> Result<Image2D<P>> {
        let buf = try!(Array2::from_shape_vec((h as usize, w as usize), v));
        Ok(Image2D{ buffer: buf })
    }

    /// Create a new image of specified dimensions from a `Vec` of the specified pixel type's
    /// subpixel.
    ///
    /// **Error**: `InvalidDimensions` if the dimensions do not match the length of `v`.
    pub fn from_raw_vec(w: u32, h: u32, v: &Vec<P::Subpixel>) -> Result<Image2D<P>> {
        let pixels_iter = v.chunks(P::n_channels());
        if pixels_iter.len() != (w * h) as usize {
            bail!(ErrorKind::InvalidDimensions);
        }
        let mut v_pixels = vec![];
        for channels in pixels_iter {
            v_pixels.push(P::from_slice(channels))
        }
        let buf = try!(Array2::from_shape_vec((h as usize, w as usize), v_pixels));
        Ok(Image2D{ buffer: buf })
    }

    /// Return the pixel at the specified coordinates.
    ///
    /// **Panics** if the index is out of bounds.
    pub fn get_pixel(&self, x: u32, y: u32) -> P {
        self.buffer[[x as usize, y as usize]].clone()
    }

    /// Set the pixel at the specified coordinates to the specified value.
    ///
    /// **Panics** if the index is out of bounds.
    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: P) {
        self.buffer[[x as usize, y as usize]] = pixel;
    }

    /// Return the width of the image.
    pub fn width(&self) -> u32 { self.buffer.cols() as u32 }
    /// Return the height of the image.
    pub fn height(&self) -> u32 { self.buffer.rows() as u32 }
    /// Return the dimensions of the image as a `(width, height)` tuple.
    pub fn dimensions(&self) -> (u32, u32) { (self.width(), self.height()) }

    // TODO: map to u32's for coherence
    /// Return an iterator to the pixels and their indices. The type of the iterator is ((usize, usize), &P)
    pub fn enumerate_pixels(&self) -> ndarray::iter::IndexedIter<P, Ix2> {
        self.buffer.indexed_iter()
    }

    // TODO: map to u32's for coherence
    /// Return an iterator to the pixels and their indices. The type of the iterator is ((usize, usize), &mut P)
    pub fn enumerate_pixels_mut(&mut self) -> ndarray::iter::IndexedIterMut<P, Ix2> {
        self.buffer.indexed_iter_mut()
    }

    /// Return an iterator on a subset of the image of specified dimensions starting at the specified
    /// coordinates.
    ///
    /// **Panics** if the specified region crosses image boundaries.
    pub fn rect_iterator(&'a self, rect: &Rect) -> Iter<P> {
        let left = rect.left() as isize;
        let top = rect.top() as isize;
        let right = left + rect.width() as isize;
        let bottom = top + rect.height() as isize;
        self.buffer.slice(s![left..right, top..bottom]).into_iter()
    }

    /// Return a mutable view to a subset of the image of specified dimensions starting at the specified
    /// coordinates.
    ///
    /// **Panics** if the specified region crosses image boundaries.
    pub fn rect_iterator_mut(&'a mut self, rect: &Rect) -> IterMut<P> {
        let left = rect.left() as isize;
        let top = rect.top() as isize;
        let right = left + rect.width() as isize;
        let bottom = top + rect.height() as isize;
        self.buffer.slice_mut(s![left..right, top..bottom]).into_iter()
    }

    /// Translate the given `Rect` within the image by the given 2D vector. The parts of the
    /// original `Rect` than fall out of the iamge will be cropped. Return the translated `Rect` if
    /// it's not empty, or `None` otherwise.
    pub fn translate_rect(&self, rect: &Rect, x: i64, y: i64) -> Option<Rect> {
        let left = (rect.left() as i64) + x;
        let top = (rect.top() as i64) + y;
        let right = (rect.right() as i64) + x;
        let bottom = (rect.bottom() as i64) + y;
        let (w_signed, h_signed) = (self.width() as i64, self.height() as i64);

        // Detect early if the resulting rectangle falls out of the image
        match left < w_signed && top < h_signed && right >= 0 && bottom >= 0 {
            true => {
                // Compute the new Rect
                let x_left = if left < 0 { 0 } else { left as u32 };
                let y_top = if top < 0 { 0 } else { top as u32 };
                Some(Rect::new(x_left, y_top, (min(w_signed, right + 1) as u32) - x_left, (min(h_signed, bottom + 1) as u32) - y_top))
            },
            false => None
        }
    }
}

impl<'a, P> IntoIterator for &'a Image2D<P>
    where P: Pixel + 'a
{
    type Item = &'a P;
    type IntoIter = Iter<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a, P> IntoIterator for &'a mut Image2D<P>
    where P: Pixel + 'a
{
    type Item = &'a mut P;
    type IntoIter = IterMut<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter_mut()
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

        let i1 = Image2D::from_vec(3, 3, v1.clone());
        let i2 = Image2D::from_vec(2, 3, v2.clone());
        let i3 = Image2D::from_vec(3, 2, v2.clone());
        assert!(i1.is_ok());
        assert!(i2.is_ok());
        assert!(i3.is_ok());
        assert_eq!(i1.unwrap().dimensions(), (3, 3));
        assert_eq!(i2.unwrap().dimensions(), (2, 3));
        assert_eq!(i3.unwrap().dimensions(), (3, 2));

        assert!(Image2D::from_vec(3, 3, v2.clone()).is_err());
        assert!(Image2D::from_vec(4, 2, v2.clone()).is_err());
    }

    #[test]
    fn test_new() {
        fn test_zeros_helper<P>(w: u32, h: u32)
            where P: Pixel + Zero + Debug
        {
            let img = Image2D::<P>::new(w, h);
            assert_eq!((w, h), (img.width(), img.height()));
            for pixel in &img {
                assert!(pixel.is_zero());
            }
        };
        test_zeros_helper::<Luma<u8>>(100, 200);
        test_zeros_helper::<Luma<f32>>(100, 200);
    }

    #[test]
    fn test_into_iter() {
        let v: Vec<Luma<u8>> = (1..10).map(|i| Luma::from(i)).collect();
        let img = Image2D::from_vec(3, 3, v.clone()).unwrap();

        for (p, i) in img.into_iter().zip(v.into_iter()) {
            assert!(&i == p);
        }
    }

    #[test]
    fn test_enumerate_pixels() {
        let mut v: Vec<Luma<u8>> = vec![];
        for x in 0..3 {
            for y in 0..3 {
                v.push(Luma::from((2*x + 3*y) as u8));
            }
        }
        let img = Image2D::from_vec(3, 3, v.clone()).unwrap();

        for ((x, y), p) in img.enumerate_pixels().map(|((x, y), p)| ((x, y), p.channels()[0])) {
            assert!((2*x + 3*y) as u8 == p);
        }
    }

    #[test]
    fn test_rect_iterator() {
        let v: Vec<Luma<usize>> = (1..10).map(|n| Luma::new([n])).collect();
        let img = Image2D::from_vec(3, 3, v).unwrap();
        let subimg1 = img.rect_iterator(&Rect::new(1, 1, 2, 2));

        fn subimg_vec_eq<'a>(subimg: Iter<'a, Luma<usize>>, v: &Vec<usize>) -> bool {
            let v_iter   = v.into_iter();
            match subimg.len() == v_iter.len() {
                true => !subimg.zip(v_iter).any(|(p, i)| p.data[0] != *i),
                false => false
            }
        }

        let subimg1_vec: Vec<usize> = vec![5, 6, 8, 9];

        assert!(subimg_vec_eq(subimg1, &subimg1_vec));
    }

    #[test]
    fn test_translate_rect() {
        let img: Image2D<Luma<u8>> = Image2D::new(5, 5);
        let r1 = Rect::new(1, 1, 3, 3);
        let r2 = Rect::new(1, 1, 5, 5);
        assert_eq!(img.translate_rect(&r1, -2, -2), Some(Rect::new(0, 0, 2, 2)));
        assert_eq!(img.translate_rect(&r1, -4, -4), None);
        assert_eq!(img.translate_rect(&r1,  2,  2), Some(Rect::new(3, 3, 2, 2)));
        assert_eq!(img.translate_rect(&r2,  2,  2), Some(Rect::new(3, 3, 2, 2)));
        assert_eq!(img.translate_rect(&r2,  0,  0), Some(Rect::new(1, 1, 4, 4)));
        assert_eq!(img.translate_rect(&r1,  4,  4), None);
    }
}
