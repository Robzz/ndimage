//! Contains the definition of the `Rect` type and trait implementations for it.

use core::{Image2D, Pixel, Region};

use std::cmp::{max, min};

/// Represent a rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Rect {
    /// Create a new `Rect`
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
        // TODO: Result
        assert!(
            w != 0 && h != 0,
            "Rect dimensions must be strictly positive."
        );
        Rect {
            left: x,
            top: y,
            width: w,
            height: h,
        }
    }

    /// Return the left coordinate of the `Rect`
    pub fn left(&self) -> u32 {
        self.left
    }

    /// Return the top coordinate of the `Rect`
    pub fn top(&self) -> u32 {
        self.top
    }

    /// Return the right coordinate of the `Rect`
    pub fn right(&self) -> u32 {
        self.left + self.width - 1
    }

    /// Return the bottom coordinate of the `Rect`
    pub fn bottom(&self) -> u32 {
        self.top + self.height - 1
    }

    /// Return the left and top coordinates of the `Rect`
    pub fn position(&self) -> (u32, u32) {
        (self.left, self.top)
    }

    /// Return the width of the `Rect`
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Return the height of the `Rect`
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Return the width and height of the `Rect`
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Return the intersection of two `Rect`s if it exists, `None` otherwise.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let left = max(self.left(), other.left());
        let top = max(self.top(), other.top());
        let right = min(self.right(), other.right());
        let bottom = min(self.bottom(), other.bottom());
        if left <= right && top <= bottom {
            let (w, h) = (right - left + 1, bottom - top + 1);
            Some(Rect::new(left, top, w, h))
        } else {
            None
        }
    }

    /// Test whether the Rect fits in the given image.
    pub fn fits_image<P>(&self, img: &Image2D<P>) -> bool
    where
        P: Pixel,
    {
        self.right() < img.width() && self.bottom() < img.height()
    }

    /// Crop the `Rect` to the biggest sub-`Rect` that can fit `img` if it exists, `None` otherwise.
    pub fn crop_to_image<P>(&self, img: &Image2D<P>) -> Option<Rect>
    where
        P: Pixel,
    {
        let r = Rect::new(0, 0, img.width(), img.height());
        self.intersection(&r)
    }
}

impl Region for Rect {
    fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.left() && y >= self.top() && x <= self.right() && y <= self.bottom()
    }
}

#[cfg(test)]
mod tests {
    use core::{ImageBuffer2D, Luma, Rect, Region};

    #[test]
    fn test_rect() {
        let r = Rect::new(5, 5, 5, 5);
        assert_eq!(r.left(), 5);
        assert_eq!(r.top(), 5);
        assert_eq!(r.right(), 9);
        assert_eq!(r.bottom(), 9);
    }

    #[test]
    fn test_intersection() {
        let r1 = Rect::new(0, 0, 150, 150);
        let r2 = Rect::new(50, 50, 150, 150);
        let r3 = Rect::new(0, 140, 150, 20);

        assert_eq!(r1.intersection(&r2), Some(Rect::new(50, 50, 100, 100)));
        assert_eq!(r1.intersection(&r3), Some(Rect::new(0, 140, 150, 10)));
    }

    #[test]
    fn test_crop_to_image() {
        let r1 = Rect::new(500, 500, 500, 500);
        let r2 = Rect::new(1000, 1000, 500, 500);
        let img: ImageBuffer2D<Luma<u8>> = ImageBuffer2D::new(800, 600);
        assert_eq!(r1.crop_to_image(&img), Some(Rect::new(500, 500, 300, 100)));
        assert_eq!(r2.crop_to_image(&img), None);
    }

    #[test]
    fn test_contains() {
        let r1 = Rect::new(500, 500, 500, 500);
        for y in 0..1500 {
            for x in 0..1500 {
                if x >= 500 && x < 1000 && y >= 500 && y < 1000 {
                    assert_eq!(r1.contains(x, y), true);
                } else {
                    assert_eq!(r1.contains(x, y), false);
                }
            }
        }
    }

    #[test]
    fn test_fits_image() {
        let r1 = Rect::new(10, 10, 32, 32);
        let r2 = Rect::new(10, 10, 64, 64);
        let r3 = Rect::new(0, 0, 64, 64);
        let r4 = Rect::new(0, 0, 65, 65);
        let r5 = Rect::new(10, 10, 54, 54);
        let r6 = Rect::new(10, 10, 55, 55);

        let img: ImageBuffer2D<Luma<u8>> = ImageBuffer2D::new(64, 64);
        assert!(r1.fits_image(&img));
        assert!(r3.fits_image(&img));
        assert!(r5.fits_image(&img));
        assert!(!r2.fits_image(&img));
        assert!(!r4.fits_image(&img));
        assert!(!r6.fits_image(&img));
    }
}
