//! Contains image padding functions.

use core::{Image2D, Image2DMut, ImageBuffer2D, Pixel, Rect};

/// Supported padding kinds.
pub enum Padding<P>
    where P: Pixel
{
    /// Constant padding.
    Constant(P),
    /// Replicate borders.
    Replicate,
    /// Wrap around borders.
    Wrap,
    /// Mirror around borders.
    Mirror
}

impl<P> Padding<P>
where
    P: Pixel
{
    /// Apply the padding to the specified image.
    pub fn apply(&self, img: &Image2D<P>, size: (u32, u32)) -> ImageBuffer2D<P> {
        match self {
            Padding::Constant(p) => pad_constant(img, size, p),
            Padding::Replicate => pad_replicate(img, size),
            Padding::Wrap => pad_wrap(img, size),
            Padding::Mirror => pad_mirror(img, size),
        }
    }
}

/// Pad an image with a constant.
pub fn pad_constant<P>(img: &Image2D<P>, size: (u32, u32), val: &P) -> ImageBuffer2D<P>
where
    P: Pixel,
{
    let (w, h) = img.dimensions();
    let mut padded = ImageBuffer2D::from_elem(w + 2 * size.0, h + 2 * size.1, val);
    let r = Rect::new(size.0, size.1, w, h);
    padded.blit_rect(img.rect(), r, img).unwrap();
    padded
}

/// Pad an image with zeros.
pub fn pad_zeros<P>(img: &Image2D<P>, size: (u32, u32)) -> ImageBuffer2D<P>
    where P: Pixel
{
    pad_constant(img, size, &P::zero())
}

/// Pad an image by replicating its borders.
pub fn pad_replicate<P>(img: &Image2D<P>, size: (u32, u32)) -> ImageBuffer2D<P>
where
    P: Pixel,
{
    let mut padded = pad_zeros(img, size);

    {
        // Fill the corners by replicating the corners and the borders by replicating the borders.
        let mut fill_corner = |x, y, val| {
            padded
                .sub_image_mut(Rect::new(x, y, size.0, size.1))
                .fill(val);
        };
        fill_corner(0, 0, img.get_pixel(0, 0));
        fill_corner(0, img.height() + size.1, img.get_pixel(0, img.height() - 1));
        fill_corner(img.width() + size.0, 0, img.get_pixel(img.width() - 1, 0));
        fill_corner(
            img.width() + size.0,
            img.height() + size.1,
            img.get_pixel(img.width() - 1, img.height() - 1),
        );
    }
    {
        // Fill the top side
        let inner_iter = img.row(0).unwrap();
        let mut outer_iter = padded.sub_image_mut(Rect::new(size.0, 0, img.width(), size.1));
        for (mut col, value) in outer_iter.cols_mut().zip(inner_iter) {
            col.fill(value.clone());
        }
    }
    {
        // Fill the left side
        let inner_iter = img.col(0).unwrap();
        let mut outer_iter = padded.sub_image_mut(Rect::new(0, size.1, size.0, img.height()));
        for (mut row, value) in outer_iter.rows_mut().zip(inner_iter) {
            row.fill(value.clone());
        }
    }
    {
        // Fill the right side
        let inner_iter = img.col(img.width() - 1).unwrap();
        let mut outer_iter = padded.sub_image_mut(Rect::new(
            img.width() + size.0,
            size.1,
            size.0,
            img.height(),
        ));
        for (mut row, value) in outer_iter.rows_mut().zip(inner_iter) {
            row.fill(value.clone());
        }
    }
    {
        // Fill the bottom side
        let inner_iter = img.row(img.height() - 1).unwrap();
        let mut outer_iter = padded.sub_image_mut(Rect::new(
            size.0,
            img.height() + size.1,
            img.width(),
            size.0,
        ));
        for (mut col, value) in outer_iter.cols_mut().zip(inner_iter) {
            col.fill(value.clone());
        }
    }

    padded
}

/// Pad an image by wrapping around its borders.
pub fn pad_wrap<P>(img: &Image2D<P>, size: (u32, u32)) -> ImageBuffer2D<P>
where
    P: Pixel,
{
    let mut padded = pad_zeros(img, size);

    {
        let mut copy_subimage = |src_rect, dst_rect| {
            padded.blit_rect(src_rect, dst_rect, img).unwrap();
        };
        copy_subimage(
            Rect::new(0, 0, size.0, size.1),
            Rect::new(img.width() + size.0, img.height() + size.1, size.0, size.1),
        );
        copy_subimage(
            Rect::new(img.width() - size.0, 0, size.0, size.1),
            Rect::new(0, img.height() + size.1, size.0, size.1),
        );
        copy_subimage(
            Rect::new(0, img.height() - size.1, size.0, size.1),
            Rect::new(img.width() + size.0, 0, size.0, size.1),
        );
        copy_subimage(
            Rect::new(img.width() - size.0, img.height() - size.1, size.0, size.1),
            Rect::new(0, 0, size.0, size.1),
        );
        copy_subimage(
            Rect::new(0, 0, img.width(), size.1),
            Rect::new(size.0, img.height() + size.1, img.width(), size.1),
        );
        copy_subimage(
            Rect::new(0, 0, size.0, img.height()),
            Rect::new(img.width() + size.0, size.1, size.0, img.height()),
        );
        copy_subimage(
            Rect::new(img.width() - size.0, 0, size.0, img.height()),
            Rect::new(0, size.1, size.0, img.height()),
        );
        copy_subimage(
            Rect::new(0, img.height() - size.1, img.width(), size.1),
            Rect::new(size.0, 0, img.width(), size.1),
        );
    }

    padded
}

/// Pad an image by mirroring its borders.
pub fn pad_mirror<P>(img: &Image2D<P>, size: (u32, u32)) -> ImageBuffer2D<P>
where
    P: Pixel,
{
    let mut padded = pad_zeros(img, size);

    {
        let mut copy_and_mirror_subimage_both = |src_rect, dst_rect| {
            let (src_subimg, mut dst_subimg) =
                (img.sub_image(src_rect), padded.sub_image_mut(dst_rect));
            for (src_rows, dst_rows) in src_subimg.rows().zip(dst_subimg.rows_mut().rev()) {
                for (src_pix, dst_pix) in src_rows.into_iter().zip(dst_rows.into_iter().rev()) {
                    *dst_pix = src_pix.clone();
                }
            }
        };
        copy_and_mirror_subimage_both(
            Rect::new(0, 0, size.0, size.1),
            Rect::new(0, 0, size.0, size.1),
        );
        copy_and_mirror_subimage_both(
            Rect::new(img.width() - size.0, 0, size.0, size.1),
            Rect::new(img.width() + size.0, 0, size.0, size.1),
        );
        copy_and_mirror_subimage_both(
            Rect::new(0, img.height() - size.1, size.0, size.1),
            Rect::new(0, img.height() + size.1, size.0, size.1),
        );
        copy_and_mirror_subimage_both(
            Rect::new(img.width() - size.0, img.height() - size.1, size.0, size.1),
            Rect::new(img.width() + size.0, img.height() + size.1, size.0, size.1),
        );
    }
    {
        let mut copy_and_mirror_subimage_hor = |src_rect, dst_rect| {
            let (src_subimg, mut dst_subimg) =
                (img.sub_image(src_rect), padded.sub_image_mut(dst_rect));
            for (src_rows, dst_rows) in src_subimg.rows().zip(dst_subimg.rows_mut()) {
                for (src_pix, dst_pix) in src_rows.into_iter().zip(dst_rows.into_iter().rev()) {
                    *dst_pix = src_pix.clone();
                }
            }
        };
        copy_and_mirror_subimage_hor(
            Rect::new(0, 0, size.0, img.height()),
            Rect::new(0, size.1, size.0, img.height()),
        );
        copy_and_mirror_subimage_hor(
            Rect::new(img.width() - size.0, 0, size.1, img.height()),
            Rect::new(img.width() + size.0, size.1, size.0, img.height()),
        );
    }
    {
        let mut copy_and_mirror_subimage_ver = |src_rect, dst_rect| {
            let (src_subimg, mut dst_subimg) =
                (img.sub_image(src_rect), padded.sub_image_mut(dst_rect));
            for (src_rows, dst_rows) in src_subimg.rows().zip(dst_subimg.rows_mut().rev()) {
                for (src_pix, dst_pix) in src_rows.into_iter().zip(dst_rows.into_iter()) {
                    *dst_pix = src_pix.clone();
                }
            }
        };
        copy_and_mirror_subimage_ver(
            Rect::new(0, 0, img.width(), size.1),
            Rect::new(size.0, 0, img.width(), size.1),
        );
        copy_and_mirror_subimage_ver(
            Rect::new(0, img.height() - size.1, img.width(), size.0),
            Rect::new(size.0, img.height() + size.1, img.width(), size.1),
        );
    }

    padded
}

#[cfg(test)]
mod tests {
    use core::padding::pad_zeros;
    use core::{Image2D, Image2DMut, ImageBuffer2D, Luma};

    use num_traits::Zero;

    #[test]
    fn test_pad_zeros() {
        let mut img = ImageBuffer2D::<Luma<u8>>::new(100, 100);
        img.fill(&Luma::new([255u8]));
        let padded_img = pad_zeros(&img, (5, 5));
        assert_eq!(padded_img.dimensions(), (110, 110));
        for ((y, x), pix) in padded_img.enumerate_pixels() {
            if x < 5 || y < 5 || x > 104 || y > 104 {
                assert_eq!(pix, &Luma::zero());
            } else {
                assert_eq!(pix, &Luma::new([255u8]));
            }
        }
    }
}
