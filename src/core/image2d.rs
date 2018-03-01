//! Defines a generic 2D image type.

use failure::Error;
use ndarray;
use ndarray::prelude::*;
use num_traits::{Zero};

use std::cmp::min;
use std::iter::{IntoIterator};

use core::{Luma, LumaA, Rgb, RgbA, Rect, Pixel, Primitive};

/// 2-dimensional image type.
pub trait Image2D<P>: Sync
    where P: Pixel
{
    /// Return a slice if the view points to contiguous memory in standard order.
    fn as_slice(&self) -> Option<&[P]>;

    /// Return the pixel at the specified coordinates.
    ///
    /// **Panics** if the index is out of bounds.
    fn get_pixel(&self, x: u32, y: u32) -> P;

    /// Return the width of the image.
    fn width(&self) -> u32;
    /// Return the height of the image.
    fn height(&self) -> u32;
    /// Return the dimensions of the image as a `(width, height)` tuple.
    fn dimensions(&self) -> (u32, u32) { (self.width(), self.height()) }

    // TODO: map to u32's for coherence
    // TODO: no more ndarray types in interface
    /// Return an iterator to the pixels and their indices. The type of the iterator is ((usize, usize), &P)
    fn enumerate_pixels(&self) -> ndarray::iter::IndexedIter<P, Ix2>;

    /// Return an iterator on a subset of the image of specified dimensions starting at the specified
    /// coordinates.
    ///
    /// **Panics** if the specified region crosses image boundaries.
    fn rect_iter(&self, rect: &Rect) -> RectIter<P>;

    /// Translate the given `Rect` within the image by the given 2D vector. The parts of the
    /// original `Rect` than fall out of the iamge will be cropped. Return the translated `Rect` if
    /// it's not empty, or `None` otherwise.
    fn translate_rect(&self, rect: &Rect, x: i64, y: i64) -> Option<Rect> {
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

    /// Return an Iterator on the image pixels.
    fn iter(&self) -> Iter<P>;

    /// Return an owned copy of the image.
    fn to_owned(&self) -> ImageBuffer2D<P>;

    /// Return a view on a rectangular region of the image.
    fn sub_image(&self, rect: &Rect) -> Image2DView<P>;
}

impl<'a, P> IntoIterator for &'a Image2D<P>
    where P: Pixel + 'a
{
    type Item = &'a P;
    type IntoIter = Iter<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Contains operations on mutable images.
pub trait Image2DMut<P>: Image2D<P>
    where P: Pixel
{
    /// Set the pixel at the specified coordinates to the specified value.
    ///
    /// **Panics** if the index is out of bounds.
    fn put_pixel(&mut self, x: u32, y: u32, pixel: P);

    // TODO: map to u32's for coherence
    // TODO: no more ndarray types in interface
    /// Return an iterator to the pixels and their indices. The type of the iterator is ((usize, usize), &mut P)
    fn enumerate_pixels_mut(&mut self) -> ndarray::iter::IndexedIterMut<P, Ix2>;

    /// Return a mutable view to a subset of the image of specified dimensions starting at the specified
    /// coordinates.
    ///
    /// **Panics** if the specified region crosses image boundaries.
    fn rect_iter_mut(&mut self, rect: &Rect) -> RectIterMut<P>;

    /// Fill the image with the given value
    fn fill(&mut self, value: P);

    /// Fill the given `Rect` with the given value.
    fn fill_rect(&mut self, rect: &Rect, value: &P) {
        for pixel in self.rect_iter_mut(rect) {
            *pixel = value.clone();
        }
    }

    /// Blit (i.e. copy) a `Rect` from the source image onto the destination image.
    fn blit_rect(&mut self, src_rect: &Rect, dst_rect: &Rect, img: &Image2D<P>) -> Result<(), Error>
        where Self: ::std::marker::Sized
    {
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

        for (src_pixel, dst_pixel) in img.rect_iter(src_rect).zip(self.rect_iter_mut(dst_rect)) {
            *dst_pixel = src_pixel.clone();
        }
        Ok(())
    }

    /// Return a mutable Iterator on the image pixels.
    fn iter_mut(&mut self) -> IterMut<P>;

    /// Return a mutable view on a rectangular region of the image.
    fn sub_image_mut(&mut self, rect: &Rect) -> Image2DViewMut<P>;
}

impl<'a, P> IntoIterator for &'a mut Image2DMut<P>
    where P: Pixel + 'a
{
    type Item = &'a mut P;
    type IntoIter = IterMut<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// Abstract representation of a 2D image. Can contain owned or borrowed data depending on the type
/// of D.
#[derive(Debug)]
pub struct Image2DRepr<D, P>
    where P: Pixel,
          D: ndarray::Data<Elem=P>
{
    buffer: ArrayBase<D, Ix2>
}

impl<D, P> PartialEq for Image2DRepr<D, P>
    where P: Pixel,
          D: ndarray::Data<Elem=P>
{
    fn eq(&self, other: &Image2DRepr<D, P>) -> bool {
        self.dimensions() == other.dimensions() && self.iter().eq(other.iter())
    }
}

unsafe impl<D, P> Sync for Image2DRepr<D, P>
    where P: Pixel,
          D: ndarray::Data<Elem=P>
{ }

impl<D, P> Image2D<P> for Image2DRepr<D, P>
    where P: Pixel,
          D: ndarray::Data<Elem=P>
{
    /// Return the width of the image.
    fn width(&self) -> u32 { self.buffer.cols() as u32 }
    /// Return the height of the image.
    fn height(&self) -> u32 { self.buffer.rows() as u32 }
    /// Return the dimensions of the image as a `(width, height)` tuple.
    fn dimensions(&self) -> (u32, u32) { (self.width(), self.height()) }

    fn as_slice(&self) -> Option<&[P]> {
        self.buffer.as_slice()
    }

    /// Return the pixel at the specified coordinates.
    ///
    /// **Panics** if the index is out of bounds.
    fn get_pixel(&self, x: u32, y: u32) -> P {
        self.buffer[[y as usize, x as usize]].clone()
    }

    // TODO: map to u32's for coherence
    /// Return an iterator to the pixels and their indices. The type of the iterator is ((usize, usize), &P)
    fn enumerate_pixels(&self) -> ndarray::iter::IndexedIter<P, Ix2> {
        self.buffer.indexed_iter()
    }

    /// Return a mutable view to a subset of the image of specified dimensions starting at the specified
    /// coordinates.
    ///
    /// **Panics** if the specified region crosses image boundaries.
    fn rect_iter(&self, rect: &Rect) -> RectIter<P> {
        let left = rect.left() as isize;
        let top = rect.top() as isize;
        let right = left + rect.width() as isize;
        let bottom = top + rect.height() as isize;

        RectIter { iter: self.buffer.slice(s![top..bottom, left..right]).into_iter() }
    }

    fn iter(&self) -> Iter<P> {
        self.buffer.into_iter()
    }

    fn to_owned(&self) -> ImageBuffer2D<P> {
        ImageBuffer2D { buffer: self.buffer.to_owned() }
    }

    fn sub_image(&self, rect: &Rect) -> Image2DView<P> {
        Image2DRepr { buffer: self.buffer.slice(s![rect.top() as usize..rect.bottom() as usize, rect.left() as usize..rect.right() as usize]) }
    }
}

impl<D, P> Image2DMut<P> for Image2DRepr<D, P>
    where P: Pixel,
          D: ndarray::DataMut<Elem=P>
{
    fn put_pixel(&mut self, x: u32, y: u32, pixel: P) {
        self.buffer[[y as usize, x as usize]] = pixel;
    }

    fn enumerate_pixels_mut(&mut self) -> ndarray::iter::IndexedIterMut<P, Ix2> {
        self.buffer.indexed_iter_mut()
    }

    fn rect_iter_mut(&mut self, rect: &Rect) -> RectIterMut<P> {
        let left = rect.left() as isize;
        let top = rect.top() as isize;
        let right = left + rect.width() as isize;
        let bottom = top + rect.height() as isize;

        RectIterMut { iter: self.buffer.slice_mut(s![top..bottom, left..right]).into_iter() }
    }

    fn fill(&mut self, value: P) {
        self.buffer.fill(value);
    }

    fn iter_mut(&mut self) -> IterMut<P> {
        self.buffer.iter_mut()
    }

    fn sub_image_mut<'a>(&'a mut self, rect: &Rect) -> Image2DViewMut<'a, P> {
        Image2DRepr { buffer: self.buffer.slice_mut(s![rect.top() as usize..rect.bottom() as usize, rect.left() as usize..rect.right() as usize]) }
    }
}

impl<'a, D, P> IntoIterator for &'a Image2DRepr<D, P>
    where P: Pixel + 'a,
          D: ndarray::DataClone<Elem=P>
{
    type Item = &'a P;
    type IntoIter = Iter<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a, D, P> IntoIterator for &'a mut Image2DRepr<D, P>
    where P: Pixel + 'a,
          D: ndarray::DataMut<Elem=P> + ndarray::DataClone<Elem=P>
{
    type Item = &'a mut P;
    type IntoIter = IterMut<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter_mut()
    }
}

/// Owned 2D image representation.
pub type ImageBuffer2D<P> = Image2DRepr<ndarray::OwnedRepr<P>, P>;
/// Borrowed 2D image representation.
pub type Image2DView<'a, P> = Image2DRepr<ndarray::ViewRepr<&'a P>, P>;
/// Mutably borrowed 2D image representation.
pub type Image2DViewMut<'a, P> = Image2DRepr<ndarray::ViewRepr<&'a mut P>, P>;

// Type of ndarray iterators.
type Iter<'a, P> = ndarray::iter::Iter<'a, P, Ix2>;
type IterMut<'a, P> = ndarray::iter::IterMut<'a, P, Ix2>;

impl<P> ImageBuffer2D<P>
    where P: Pixel
{
    /// Create a new owned image of specified dimensions filled with zeros.
    pub fn new(width: u32, height: u32) -> ImageBuffer2D<P>
        where P: Pixel + Zero
    {
        ImageBuffer2D { buffer: Array2::zeros((height as usize, width as usize)) }
    }

    /// Consume self and return the raw underlying storage Vec.
    pub fn into_raw_vec(self) -> Vec<P> {
        self.buffer.into_raw_vec()
    }

    /// Create a new image of specified dimensions from a `Vec` of the specified pixel type.
    ///
    /// **Error**: `InvalidDimensions` if the dimensions do not match the length of `v`.
    pub fn from_vec(w: u32, h: u32, v: Vec<P>) -> Result<ImageBuffer2D<P>, Error> {
        let buf = try!(Array2::from_shape_vec((h as usize, w as usize), v));
        Ok(ImageBuffer2D { buffer: buf })
    }

    /// Create a new image of specified dimensions from a `Vec` of the specified pixel type's
    /// subpixel.
    ///
    /// **Error**: `InvalidDimensions` if the dimensions do not match the length of `v`.
    pub fn from_raw_vec(w: u32, h: u32, v: &[P::Subpixel]) -> Result<ImageBuffer2D<P>, Error> {
        let pixels_iter = v.chunks(P::N_CHANNELS as usize);
        ensure!(pixels_iter.len() == (w * h) as usize,
                "Buffer has incorrect size {}, expected {}.", pixels_iter.len(), w * h);
        let mut v_pixels = vec![];
        for channels in pixels_iter {
            v_pixels.push(P::from_slice(channels))
        }
        let buf = try!(Array2::from_shape_vec((h as usize, w as usize), v_pixels));
        Ok(ImageBuffer2D { buffer: buf })
    }
}

/// Iterator over a rectangular region. Created by `Image2D`'s `rect_iter` method.
pub struct RectIter<'a, P>
    where P: Pixel + 'a
{
    iter: Iter<'a, P>
}

impl<'a, P> Iterator for RectIter<'a, P>
    where P: Pixel + 'a
{
    type Item = &'a P;

    fn next(&mut self) -> Option<&'a P> {
        self.iter.next()
    }
}

/// Mutable iterator over a rectangular region. Created by `Image2DMut`'s `rect_iter_mut` method.
pub struct RectIterMut<'a, P>
    where P: Pixel + 'a
{
    iter: IterMut<'a, P>
}

impl<'a, P> Iterator for RectIterMut<'a, P>
    where P: Pixel + 'a
{
    type Item = &'a mut P;

    fn next(&mut self) -> Option<&'a mut P> {
        self.iter.next()
    }
}
/// Discard the alpha component of an `RgbA` image.
pub fn rgba_to_rgb<P>(img: &Image2D<RgbA<P>>) -> ImageBuffer2D<Rgb<P>>
    where P: Primitive
{
    let mut res = ImageBuffer2D::<Rgb<P>>::new(img.width(), img.height());
    for (src_pixel, dst_pixel) in img.into_iter().zip((&mut res).into_iter()) {
        *dst_pixel = src_pixel.into();
    }
    res
}

/// Discard the alpha component of a `LumaA` image.
pub fn luma_alpha_to_luma<P>(img: &Image2D<LumaA<P>>) -> ImageBuffer2D<Luma<P>>
    where P: Primitive
{
    let mut res = ImageBuffer2D::<Luma<P>>::new(img.width(), img.height());
    for (src_pixel, dst_pixel) in img.into_iter().zip((&mut res).into_iter()) {
        *dst_pixel = src_pixel.into();
    }
    res
}

#[cfg(test)]
mod tests {
    use core::{Image2D, Image2DMut, ImageBuffer2D, Region, Pixel, Luma, Rect};

    use num_traits::Zero;

    use std::iter::FromIterator;
    use std::fmt::Debug;

    #[test]
    fn test_from_vec() {
        let v1 = Vec::from_iter((0u8..9u8).map(|n| Luma::new([n])));
        let v2 = Vec::from_iter((0u8..6u8).map(|n| Luma::new([n])));

        let i1 = ImageBuffer2D::from_vec(3, 3, v1.clone()).unwrap();
        let i2 = ImageBuffer2D::from_vec(2, 3, v2.clone()).unwrap();
        let i3 = ImageBuffer2D::from_vec(3, 2, v2.clone()).unwrap();
        assert_eq!(i1.dimensions(), (3, 3));
        assert_eq!(i2.dimensions(), (2, 3));
        assert_eq!(i3.dimensions(), (3, 2));

        assert!(ImageBuffer2D::from_vec(3, 3, v2.clone()).is_err());
        assert!(ImageBuffer2D::from_vec(4, 2, v2.clone()).is_err());
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
            let img = ImageBuffer2D::<P>::new(w, h);
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
        let img = ImageBuffer2D::from_vec(3, 3, v.clone()).unwrap();

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
        let img = ImageBuffer2D::from_vec(5, 3, v.clone()).unwrap();

        for ((x, y), p) in img.enumerate_pixels().map(|((y, x), p)| ((x, y), p.channels()[0])) {
            assert_eq!((2*x + 3*y) as u8, p);
        }
    }

    #[test]
    fn test_rect_iter() {
        let v: Vec<Luma<u8>> = (1_u8..16_u8).map(|n| Luma::new([n])).collect();
        let img = ImageBuffer2D::from_vec(5, 3, v).unwrap();
        let subimg1 = img.rect_iter(&Rect::new(1, 1, 3, 1));

        fn subimg_vec_eq<'a>(subimg: super::RectIter<'a, Luma<u8>>, v: &Vec<u8>) -> bool {
            v.into_iter().zip(subimg).all(|(p, l)| *p == l.data[0])
        }

        let subimg1_vec: Vec<u8> = vec![7, 8, 9];

        assert!(subimg_vec_eq(subimg1, &subimg1_vec));
    }

    #[test]
    fn test_translate_rect() {
        let img: ImageBuffer2D<Luma<u8>> = ImageBuffer2D::new(5, 5);
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
        let mut img: ImageBuffer2D<Luma<u8>> = ImageBuffer2D::new(5, 5);
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
        let mut img1 = ImageBuffer2D::<Luma<u8>>::new(64, 64);
        let mut img2 = ImageBuffer2D::<Luma<u8>>::new(64, 64);
        let r = Rect::new(16, 16, 32, 32);
        img2.fill_rect(&r, &Luma::<u8>::new([255]));
        assert!(img1.blit_rect(&r, &r, &img2).is_ok());
        assert_eq!(img1, img2);
    }

    #[test]
    fn test_sub_image() {
        let mut v: Vec<Luma<u8>> = vec![];
        for y in 0..5 {
            for x in 0..5 {
                v.push(Luma::from((2*x + 3*y) as u8));
            }
        }
        let img = ImageBuffer2D::from_vec(5, 5, v.clone()).unwrap();
        let sub_img = img.sub_image(&Rect::new(1, 1, 3, 3));

        for ((x, y), p) in sub_img.enumerate_pixels().map(|((y, x), p)| ((x, y), p.channels()[0])) {
            assert_eq!((2*(x+1) + 3*(y+1)) as u8, p);
        }
    }
}
