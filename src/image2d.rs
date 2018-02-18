//! Defines a generic 2D image type.

use failure::Error;
use ndarray;
use ndarray::prelude::*;
use num_traits::{Zero};

use std::cmp::min;
use std::iter::{IntoIterator};

use pixel_types::{Luma, LumaA, Rgb, RgbA};
use rect::Rect;
use traits::{Pixel, Primitive};

/// 2-dimensional image type.
#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn from_vec(w: u32, h: u32, v: Vec<P>) -> Result<Image2D<P>, Error> {
        let buf = try!(Array2::from_shape_vec((h as usize, w as usize), v));
        Ok(Image2D { buffer: buf })
    }

    /// Create a new image of specified dimensions from a `Vec` of the specified pixel type's
    /// subpixel.
    ///
    /// **Error**: `InvalidDimensions` if the dimensions do not match the length of `v`.
    pub fn from_raw_vec(w: u32, h: u32, v: &[P::Subpixel]) -> Result<Image2D<P>, Error> {
        let pixels_iter = v.chunks(P::N_CHANNELS as usize);
        ensure!(pixels_iter.len() == (w * h) as usize,
                "Buffer has incorrect size {}, expected {}.", pixels_iter.len(), w * h);
        let mut v_pixels = vec![];
        for channels in pixels_iter {
            v_pixels.push(P::from_slice(channels))
        }
        let buf = try!(Array2::from_shape_vec((h as usize, w as usize), v_pixels));
        Ok(Image2D { buffer: buf })
    }

    pub fn into_raw_vec(self) -> Vec<P> {
        self.buffer.into_raw_vec()
    }

    pub fn as_slice(&self) -> Option<&[P]> {
        self.buffer.as_slice()
    }

    /// Return the pixel at the specified coordinates.
    ///
    /// **Panics** if the index is out of bounds.
    pub fn get_pixel(&self, x: u32, y: u32) -> P {
        self.buffer[[y as usize, x as usize]].clone()
    }

    /// Set the pixel at the specified coordinates to the specified value.
    ///
    /// **Panics** if the index is out of bounds.
    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: P) {
        self.buffer[[y as usize, x as usize]] = pixel;
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

    /// Fill the image with the given value
    pub fn fill(&'a mut self, value: P) {
        self.buffer.fill(value);
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
        self.buffer.slice(s![top..bottom, left..right]).into_iter()
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
        self.buffer.slice_mut(s![top..bottom, left..right]).into_iter()
    }

    /// Translate the given `Rect` within the image by the given 2D vector. The parts of the
    /// original `Rect` than fall out of the iamge will be cropped. Return the translated `Rect` if
    /// it's not empty, or `None` otherwise.
    pub fn translate_rect(&self, rect: &Rect, x: i64, y: i64) -> Option<Rect> {
        let left = i64::from(rect.left()) + x;
        let top = i64::from(rect.top()) + y;
        let right = i64::from(rect.right()) + x;
        let bottom = i64::from(rect.bottom()) + y;
        let (w_signed, h_signed) = (i64::from(self.width()), i64::from(self.height()));

        // Detect early if the resulting rectangle falls out of the image
        if left < w_signed && top < h_signed && right >= 0 && bottom >= 0 {
            // Compute the new Rect
            let x_left = if left < 0 { 0 } else { left as u32 };
            let y_top = if top < 0 { 0 } else { top as u32 };
            Some(Rect::new(x_left, y_top, (min(w_signed, right + 1) as u32) - x_left, (min(h_signed, bottom + 1) as u32) - y_top))
        }
        else { None }
    }

    /// Fill the given `Rect` with the given value.
    pub fn fill_rect(&'a mut self, rect: &Rect, value: &P) {
        for pixel in self.rect_iterator_mut(rect) {
            *pixel = value.clone();
        }
    }

    /// Blit (i.e. copy) a `Rect` from the source image onto the destination image.
    pub fn blit_rect(&'a mut self, src_rect: &Rect, dst_rect: &Rect, img: &Image2D<P>) -> Result<(), Error> {
        if src_rect.size() != dst_rect.size() {
            let (ws, hs) = src_rect.size();
            let (wd, hd) = dst_rect.size();
            bail!("Rects are not the same size. Source is ({}, {}), destination is ({}, {})", ws, hs, wd, hd);
        }

        if !src_rect.fits_image(img) {
            bail!("Source rect does not fit source image.");
        }
        if !dst_rect.fits_image(self) {
            bail!("Source rect does not fit destination image.");
        }

        for (src_pixel, dst_pixel) in img.rect_iterator(src_rect).zip(self.rect_iterator_mut(dst_rect)) {
            *dst_pixel = src_pixel.clone();
        }
        Ok(())
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

/// Discard the alpha component of an RgbA image.
pub fn rgba_to_rgb<P>(img: &Image2D<RgbA<P>>) -> Image2D<Rgb<P>>
    where P: Primitive
{
    let mut res = Image2D::<Rgb<P>>::new(img.width(), img.height());
    for (src_pixel, dst_pixel) in img.into_iter().zip((&mut res).into_iter()) {
        *dst_pixel = src_pixel.into();
    }
    res
}

/// Discard the alpha component of a LumaA image.
pub fn luma_alpha_to_luma<P>(img: &Image2D<LumaA<P>>) -> Image2D<Luma<P>>
    where P: Primitive
{
    let mut res = Image2D::<Luma<P>>::new(img.width(), img.height());
    for (src_pixel, dst_pixel) in img.into_iter().zip((&mut res).into_iter()) {
        *dst_pixel = src_pixel.into();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::Luma;

    use std::iter::FromIterator;
    use std::fmt::Debug;

    #[test]
    fn test_from_vec() {
        let v1 = Vec::from_iter((0u8..9u8).map(|n| Luma::new([n])));
        let v2 = Vec::from_iter((0u8..6u8).map(|n| Luma::new([n])));

        let i1 = Image2D::from_vec(3, 3, v1.clone()).unwrap();
        let i2 = Image2D::from_vec(2, 3, v2.clone()).unwrap();
        let i3 = Image2D::from_vec(3, 2, v2.clone()).unwrap();
        assert_eq!(i1.dimensions(), (3, 3));
        assert_eq!(i2.dimensions(), (2, 3));
        assert_eq!(i3.dimensions(), (3, 2));

        assert!(Image2D::from_vec(3, 3, v2.clone()).is_err());
        assert!(Image2D::from_vec(4, 2, v2.clone()).is_err());
        for y in 0..3 {
            for x in 0..2 {
                assert_eq!((x + y * 2) as u8, i2.get_pixel(x, y).data[0]);
            }
        }
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
        for y in 0..3 {
            for x in 0..5 {
                v.push(Luma::from((2*x + 3*y) as u8));
            }
        }
        let img = Image2D::from_vec(5, 3, v.clone()).unwrap();

        for ((x, y), p) in img.enumerate_pixels().map(|((y, x), p)| ((x, y), p.channels()[0])) {
            assert_eq!((2*x + 3*y) as u8, p);
        }
    }

    #[test]
    fn test_rect_iterator() {
        let v: Vec<Luma<usize>> = (1..16).map(|n| Luma::new([n])).collect();
        let img = Image2D::from_vec(5, 3, v).unwrap();
        let subimg1 = img.rect_iterator(&Rect::new(1, 1, 3, 1));

        fn subimg_vec_eq<'a>(subimg: Iter<'a, Luma<usize>>, v: &Vec<usize>) -> bool {
            let v_iter   = v.into_iter();
            match subimg.len() == v_iter.len() {
                true => !subimg.zip(v_iter).any(|(p, i)| p.data[0] != *i),
                false => false
            }
        }

        let subimg1_vec: Vec<usize> = vec![7, 8, 9];

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

    #[test]
    fn test_fill_rect() {
        use traits::Region;

        let mut img: Image2D<Luma<u8>> = Image2D::new(5, 5);
        let r = Rect::new(1, 1, 3, 3);
        img.fill_rect(&r, &Luma::<u8>::new([255]));
        for ((x, y), &pixel) in img.enumerate_pixels() {
            if r.contains(x as u32, y as u32) {
                assert_eq!(pixel, Luma::<u8>::new([255]));
            }
            else {
                assert_eq!(pixel, Luma::<u8>::new([0]));
            }
        }
    }

    #[test]
    fn test_blit_rect() {
        let mut img1 = Image2D::<Luma<u8>>::new(64, 64);
        let mut img2 = Image2D::<Luma<u8>>::new(64, 64);
        let r = Rect::new(16, 16, 32, 32);
        img2.fill_rect(&r, &Luma::<u8>::new([255]));
        assert!(img1.blit_rect(&r, &r, &img2).is_ok());
        assert_eq!(img1, img2);
    }
}
