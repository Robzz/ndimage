//! Harris and Shi-Tomasi corner detectors.

use core::{
    padding::{pad_mirror, Padding},
    Image2D, ImageBuffer2D, Luma, Primitive, Rect
};
//use io::save;
use math::gaussian_2d;
use processing::kernel::Kernel;

use num_traits::{NumCast, Zero};

/// Detect corners in a grayscale image with the Harris corner detector.
pub fn harris_corners<P>(img: &Image2D<Luma<P>>, radius: u32, k: f64) -> Vec<(u32, u32)>
where
    P: Primitive + Zero
{
    // Compute the image derivatives
    let gaussian = Kernel::<f64>::gaussian(radius as f64, radius);
    let sobel_x = Kernel::<f64>::sobel_x_3x3();
    let sobel_y = Kernel::<f64>::sobel_y_3x3();

    let padded = pad_mirror(img, radius);
    let blurred = gaussian.convolve(&padded, Padding::Mirror);
    let dx = sobel_x
        .convolve::<Luma<P>, Luma<f64>, P, f64>(&blurred, Padding::Zero)
        .abs();
    //save("sobel_x.png", dx);
    let dy = sobel_y
        .convolve::<Luma<P>, Luma<f64>, P, f64>(&blurred, Padding::Zero)
        .abs();

    let len = 2 * radius + 1;
    let n_pixels = len * len;

    let mut window = Vec::with_capacity(n_pixels as usize);
    for i in 0i32..n_pixels as i32 {
        let (x, y) = (
            ((i % len as i32) - radius as i32) as f64,
            ((i / len as i32) - radius as i32) as f64
        );
        let g = gaussian_2d(x, y, radius as f64);
        window.push(g);
    }
    // Compute the Harris response
    let harris = ImageBuffer2D::<Luma<f64>>::generate(img.width(), img.height(), |(x, y)| {
        let rect = Rect::new(x, y, len, len);
        let mut m = [0., 0., 0., 0.];
        for ((ix, iy), w) in dx
            .rect_iter(rect)
            .zip(dy.rect_iter(rect))
            .zip(window.iter())
        {
            let (ix_f64, iy_f64) = (
                <f64 as NumCast>::from(ix[0]).unwrap(),
                <f64 as NumCast>::from(iy[0]).unwrap()
            );
            let a = ix_f64 * ix_f64 * w;
            let b = iy_f64 * iy_f64 * w;
            let c = ix_f64 * iy_f64 * w;
            m[0] += a;
            m[1] += c;
            m[2] += c;
            m[3] += b;
        }
        let det = m[0] * m[3] - m[2] * m[1];
        let tr = m[0] + m[3];
        let e = det - k * tr * tr;
        Luma::new([e])
    });

    // TODO: extract function
    // Find positive local maxima
    let mut corners = Vec::new();
    let rect = Rect::new(1, 1, img.width() - 2, img.height() - 2);

    let pw = img.width() - 2;
    for (idx, pix) in harris.rect_iter(rect).enumerate() {
        let e = pix[0];
        if e > 10_000. {
            let (x, y) = (idx as u32 % pw + 1, idx as u32 / pw + 1);
            if harris[(x - 1, y - 1)][0] < e
                && harris[(x - 1, y)][0] < e
                && harris[(x - 1, y + 1)][0] < e
                && harris[(x, y - 1)][0] < e
                && harris[(x, y + 1)][0] < e
                && harris[(x + 1, y - 1)][0] < e
                && harris[(x + 1, y)][0] < e
                && harris[(x + 1, y + 1)][0] < e
            {
                corners.push((x, y));
            }
        }
    }

    corners
}
