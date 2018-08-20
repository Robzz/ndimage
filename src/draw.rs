//! Drawing functions.

use core::{Image2DMut, Pixel};

/// Draw a `+` shaped cross.
pub fn draw_cross<P>(img: &mut Image2DMut<P>, coords: (u32, u32), size: u32, color: P)
where
    P: Pixel
{
    let (w, h) = img.dimensions();
    if coords.0 < w && coords.1 < h {
        let compute_bounds = |c: u32| {
            let cs = c.checked_sub(size).unwrap_or(0);
            let mut ce = c + size;
            if ce >= w {
                ce = w - 1;
            }
            (cs, ce)
        };
        let (xs, xe) = compute_bounds(coords.0);
        let (ys, ye) = compute_bounds(coords.1);
        for pix in img
            .row_mut(coords.1)
            .unwrap()
            .skip(xs as usize)
            .take((xe - xs + 1) as usize)
        {
            *pix = color.clone();
        }
        for pix in img
            .col_mut(coords.0)
            .unwrap()
            .skip(ys as usize)
            .take((ye - ys + 1) as usize)
        {
            *pix = color.clone();
        }
    }
}
