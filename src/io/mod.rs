//! Contains modules related to image I/O.

#[macro_use] mod macros;
pub mod png;
pub mod tiff;
pub mod traits;

use self::{
    png::PngEncodable,
    traits::ImageDecoder
};
use core::{DynamicImage, Pixel, Image2D};

use failure::Error;

use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerate the image formats supported by the library.
pub enum Format {
    /// PNG format.
    Png,
    /// TIFF format.
    Tiff,
}

fn parse_extension<P>(filepath: &P) -> Option<Format>
where
    P: AsRef<Path>,
{
    let ext = filepath
        .as_ref()
        .extension()?
        .to_string_lossy()
        .to_ascii_lowercase();
    match ext.as_str() {
        "tiff" => Some(Format::Tiff),
        "png" => Some(Format::Png),
        _ => None,
    }
}

/// Open an image on the filesystem. Try to guess the image format from the file extension.
pub fn open<P>(filepath: P) -> Result<DynamicImage, Error>
where
    P: AsRef<Path>,
{
    if let Some(format) = parse_extension(&filepath) {
        let file = File::open(filepath)?;
        match format {
            Format::Png => png::Decoder::new(file)?.read_image(),
            Format::Tiff => tiff::Decoder::new(file)?.read_image(),
        }
    } else {
        bail!("Could not infer image format from file extension!")
    }
}

/// Save an image to the disk. Try to guess the image format from the file extension.
pub fn save<I, P, P2>(filepath: P2, img: &I) -> Result<(), Error>
where
    I: Image2D<P>,
    P: Pixel + PngEncodable<P>,
    P2: AsRef<Path>
{
    if let Some(format) = parse_extension(&filepath) {
        match format {
            Format::Tiff => {
                bail!("TIFF encoding is not supported yet.");
            },
            Format::Png => {
                let out = File::create(filepath)?;
                <P as PngEncodable<P>>::write_image(out, img)
            }
        }
    }
    else {
        bail!("Could not infer image format from file extension!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::{BitDepth, PixelType, ImageBuffer2D, Primitive, Luma, Rgb, Image2DMut};

    use num_traits::{Zero, NumCast};
    use tempfile::tempdir;

    use std::fmt::Debug;

    #[test]
    fn test_parse_extension() {
        assert_eq!(parse_extension(&"img.tiff".to_owned()), Some(Format::Tiff));
        assert_eq!(parse_extension(&"img.png".to_owned()), Some(Format::Png));
        assert_eq!(parse_extension(&"img.TIFF".to_owned()), Some(Format::Tiff));
    }

    #[test]
    fn test_open_png() {
        let img_luma_u8 = open("./test_data/io/png/grayscale_8bit.png").unwrap();
        assert_eq!(img_luma_u8.image_type(), (PixelType::Luma, BitDepth::_8));
        assert!(img_luma_u8.is_luma());
        assert!(img_luma_u8.as_luma_u8().is_ok());
        let img_luma_u16 = open("./test_data/io/png/grayscale_16bit.png").unwrap();
        assert_eq!(img_luma_u16.image_type(), (PixelType::Luma, BitDepth::_16));
        assert!(img_luma_u16.is_luma());
        assert!(img_luma_u16.as_luma_u16().is_ok());
        let img_luma_alpha_u8 = open("./test_data/io/png/grayscale_alpha_8bit.png").unwrap();
        assert_eq!(
            img_luma_alpha_u8.image_type(),
            (PixelType::LumaA, BitDepth::_8)
        );
        assert!(img_luma_alpha_u8.is_luma_alpha());
        assert!(img_luma_alpha_u8.as_luma_alpha_u8().is_ok());
        let img_luma_alpha_u16 = open("./test_data/io/png/grayscale_alpha_16bit.png").unwrap();
        assert_eq!(
            img_luma_alpha_u16.image_type(),
            (PixelType::LumaA, BitDepth::_16)
        );
        assert!(img_luma_alpha_u16.is_luma_alpha());
        assert!(img_luma_alpha_u16.as_luma_alpha_u16().is_ok());
        let img_rgb_u8 = open("./test_data/io/png/rgb_8bit.png").unwrap();
        assert_eq!(img_rgb_u8.image_type(), (PixelType::Rgb, BitDepth::_8));
        assert!(img_rgb_u8.is_rgb());
        assert!(img_rgb_u8.as_rgb_u8().is_ok());
        let img_rgb_u16 = open("./test_data/io/png/rgb_16bit.png").unwrap();
        assert_eq!(img_rgb_u16.image_type(), (PixelType::Rgb, BitDepth::_16));
        assert!(img_rgb_u16.is_rgb());
        assert!(img_rgb_u16.as_rgb_u16().is_ok());
        let img_rgb_alpha_u8 = open("./test_data/io/png/rgba_8bit.png").unwrap();
        assert_eq!(
            img_rgb_alpha_u8.image_type(),
            (PixelType::RgbA, BitDepth::_8)
        );
        assert!(img_rgb_alpha_u8.is_rgb_alpha());
        assert!(img_rgb_alpha_u8.as_rgb_alpha_u8().is_ok());
        let img_rgb_alpha_u16 = open("./test_data/io/png/rgba_16bit.png").unwrap();
        assert_eq!(
            img_rgb_alpha_u16.image_type(),
            (PixelType::RgbA, BitDepth::_16)
        );
        assert!(img_rgb_alpha_u16.is_rgb_alpha());
        assert!(img_rgb_alpha_u16.as_rgb_alpha_u16().is_ok());
    }

    #[test]
    fn test_open_tiff() {
        let img_luma_u8 = open("./test_data/io/tiff/grayscale_8bit.tiff").unwrap();
        assert_eq!(img_luma_u8.image_type(), (PixelType::Luma, BitDepth::_8));
        assert!(img_luma_u8.is_luma());
        assert!(img_luma_u8.as_luma_u8().is_ok());
        let img_luma_u16 = open("./test_data/io/tiff/grayscale_16bit.tiff").unwrap();
        assert_eq!(img_luma_u16.image_type(), (PixelType::Luma, BitDepth::_16));
        assert!(img_luma_u16.is_luma());
        assert!(img_luma_u16.as_luma_u16().is_ok());
        //let img_luma_alpha_u8 = open("./test_data/io/tiff/grayscale_alpha_8bit.tiff").unwrap();
        //assert_eq!(img_luma_alpha_u8.image_type(), (PixelType::LumaA, BitDepth::_8));
        //assert!(img_luma_alpha_u8.is_luma_alpha());
        //assert!(img_luma_alpha_u8.as_luma_alpha_u8().is_ok());
        //let img_luma_alpha_u16 = open("./test_data/io/tiff/grayscale_alpha_16bit.tiff").unwrap();
        //assert_eq!(img_luma_alpha_u16.image_type(), (PixelType::LumaA, BitDepth::_16));
        //assert!(img_luma_alpha_u16.is_luma_alpha());
        //assert!(img_luma_alpha_u16.as_luma_alpha_u16().is_ok());
        let img_rgb_u8 = open("./test_data/io/tiff/rgb_8bit.tiff").unwrap();
        assert_eq!(img_rgb_u8.image_type(), (PixelType::Rgb, BitDepth::_8));
        assert!(img_rgb_u8.is_rgb());
        assert!(img_rgb_u8.as_rgb_u8().is_ok());
        let img_rgb_u16 = open("./test_data/io/tiff/rgb_16bit.tiff").unwrap();
        assert_eq!(img_rgb_u16.image_type(), (PixelType::Rgb, BitDepth::_16));
        assert!(img_rgb_u16.is_rgb());
        assert!(img_rgb_u16.as_rgb_u16().is_ok());
        let img_rgb_alpha_u8 = open("./test_data/io/tiff/rgba_8bit.tiff").unwrap();
        assert_eq!(
            img_rgb_alpha_u8.image_type(),
            (PixelType::RgbA, BitDepth::_8)
        );
        assert!(img_rgb_alpha_u8.is_rgb_alpha());
        assert!(img_rgb_alpha_u8.as_rgb_alpha_u8().is_ok());
        let img_rgb_alpha_u16 = open("./test_data/io/tiff/rgba_16bit.tiff").unwrap();
        assert_eq!(
            img_rgb_alpha_u16.image_type(),
            (PixelType::RgbA, BitDepth::_16)
        );
        assert!(img_rgb_alpha_u16.is_rgb_alpha());
        assert!(img_rgb_alpha_u16.as_rgb_alpha_u16().is_ok());
    }

    fn mk_test_img<P, S>() -> ImageBuffer2D<P>
    where
        P: Pixel<Subpixel = S> + Zero,
        S: Primitive + Sized,
    {
        let mut img = ImageBuffer2D::new(32, 32);
        for y in 0..32 {
            for x in 0..32 {
                let n = <S as NumCast>::from::<u32>(x + y).unwrap();
                let pix = vec![n; P::N_CHANNELS as usize];
                img.put_pixel(x, y, P::from_slice(&pix));
            }
        }
        img
    }

    fn helper_test_write_roundtrip_u8<F, P, P2>(path: P2, img: ImageBuffer2D<P>, fn_decode: F)
    where
        F: FnOnce(P2) -> Result<Box<ImageBuffer2D<P>>, Error>,
        P: Pixel<Subpixel = u8> + Debug + PngEncodable<P>,
        P2: AsRef<Path>
    {
        {
            save(&path, &img).unwrap();
        }
        let img2 = fn_decode(path).unwrap();
        assert_eq!(&img, img2.as_ref());
    }

    fn helper_test_write_roundtrip_u16<F, P, P2>(path: P2, img: ImageBuffer2D<P>, fn_decode: F)
    where
        F: FnOnce(P2) -> Result<Box<ImageBuffer2D<P>>, Error>,
        P: Pixel<Subpixel = u16> + Debug + PngEncodable<P>,
        P2: AsRef<Path>
    {
        {
            save(&path, &img).unwrap();
        }
        let img2 = fn_decode(path).unwrap();
        assert_eq!(&img, img2.as_ref());
    }

    #[test]
    fn test_save_png() {
        let dir = tempdir().unwrap();
        let img_luma_u8 = mk_test_img::<Luma<u8>, u8>();
        let img_luma_u16 = mk_test_img::<Luma<u16>, u16>();
        let img_luma_alpha_u8 = mk_test_img::<Luma<u8>, u8>();
        let img_luma_alpha_u16 = mk_test_img::<Luma<u16>, u16>();
        let img_rgb_u8 = mk_test_img::<Rgb<u8>, u8>();
        let img_rgb_u16 = mk_test_img::<Rgb<u16>, u16>();
        let img_rgb_alpha_u8 = mk_test_img::<Rgb<u8>, u8>();
        let img_rgb_alpha_u16 = mk_test_img::<Rgb<u16>, u16>();
        helper_test_write_roundtrip_u8(dir.path().join("test_save_png_luma_u8.png"), img_luma_u8, |p| open(p)?.as_luma_u8());
        helper_test_write_roundtrip_u16(dir.path().join("test_save_png_luma_u16.png"), img_luma_u16, |p| open(p)?.as_luma_u16());
        helper_test_write_roundtrip_u8(dir.path().join("test_save_png_luma_alpha_u8.png"), img_luma_alpha_u8, |p| open(p)?.as_luma_u8());
        helper_test_write_roundtrip_u16(dir.path().join("test_save_png_luma_alpha_u16.png"), img_luma_alpha_u16, |p| open(p)?.as_luma_u16());
        helper_test_write_roundtrip_u8(dir.path().join("test_save_png_rgb_u8.png"), img_rgb_u8, |p| open(p)?.as_rgb_u8());
        helper_test_write_roundtrip_u16(dir.path().join("test_save_png_rgb_u16.png"), img_rgb_u16, |p| open(p)?.as_rgb_u16());
        helper_test_write_roundtrip_u8(dir.path().join("test_save_png_rgb_alpha_u8.png"), img_rgb_alpha_u8, |p| open(p)?.as_rgb_u8());
        helper_test_write_roundtrip_u16(dir.path().join("test_save_png_rgb_alpha_u16.png"), img_rgb_alpha_u16, |p| open(p)?.as_rgb_u16());
    }
}
