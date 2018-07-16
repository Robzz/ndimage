//! Harris and Shi-Tomasi corner detectors.

use core::{Image2D, ImageBuffer2D, Luma, Primitive, Rect, padding::pad_mirror};
use math::gaussian_2d;
use processing::kernel::Kernel;

use num_traits::{Zero, NumCast};

/// Detect corners in a grayscale image with the Harris corner detector.
pub fn harris_corners<P>(img: &Image2D<Luma<P>>, winsize: u32, k: f64) -> Vec<(u32, u32)>
where
    P: Primitive + Zero
{
    // Compute the image derivatives
    let sobel_x = Kernel::<f64>::sobel_x_3x3();
    let sobel_y = Kernel::<f64>::sobel_y_3x3();

    let dx = sobel_x.convolve(img);
    let dy = sobel_y.convolve(img);

    let n_pixels = 2 * winsize + 1;

    // Compute the Harris response
    let padded = pad_mirror(img, winsize);
    let harris = ImageBuffer2D::<Luma<f64>>::generate(img.width(), img.height(),
        |(x, y)| {
            let rect = Rect::new(x + winsize, y + winsize, winsize, winsize);
            let mut e = 0.;
            let mut window = Vec::with_capacity(n_pixels as usize);
            for i in 0..n_pixels {
                let (x, y) = ((i % n_pixels - winsize) as f64, (i / n_pixels - winsize) as f64);
                window.push(gaussian_2d(x, y, 1.));
            }
            for (((pix, ix), iy), w) in padded.rect_iter(rect)
                                  .zip(dx.into_iter())
                                  .zip(dy.into_iter())
                                  .zip(window.into_iter())
            {
                let (ix_f64, iy_f64) = (<f64 as NumCast>::from(ix[0]).unwrap(), <f64 as NumCast>::from(iy[0]).unwrap());
                let a = ix_f64 * ix_f64 * w;
                let b = iy_f64 * iy_f64 * w;
                let c = ix_f64 * iy_f64 * w;
                let det = a * b - c * c;
                let tr = a + b;
                e += det - k * tr;
            }
            Luma::new([e])
        }
    );

    // Find positive local maxima
    let mut corners = Vec::new();
    let rect = Rect::new(1, 1, img.width() - 2, img.height() - 2);

    for (idx, pix) in harris.rect_iter(rect).enumerate() {
        let e = pix[0];
        if e > 0. {
            let (x, y) = (idx as u32 % n_pixels + 1, idx as u32 / n_pixels + 1);
            if harris[(x - 1, y - 1)][0] < e && harris[(x - 1, y)][0] < e &&
               harris[(x - 1, y + 1)][0] < e && harris[(x, y - 1)][0] < e &&
               harris[(x, y + 1)][0] < e && harris[(x + 1, y - 1)][0] < e &&
               harris[(x + 1, y)][0] < e && harris[(x + 1, y + 1)][0] < e
            {
                corners.push((x, y));
            }
        }
    }

    corners
}
