//! Contains modules related to image I/O.

use self::traits::ImageDecoder;
use core::DynamicImage;

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

pub mod png;
pub mod tiff;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::*;

    use core::{BitDepth, Channels};

    #[test]
    fn test_parse_extension() {
        assert_eq!(parse_extension(&"img.tiff".to_owned()), Some(Format::Tiff));
        assert_eq!(parse_extension(&"img.png".to_owned()), Some(Format::Png));
        assert_eq!(parse_extension(&"img.TIFF".to_owned()), Some(Format::Tiff));
    }

    #[test]
    fn test_open_png() {
        let img_luma_u8 = open("./test_data/io/png/grayscale_8bit.png").unwrap();
        assert_eq!(img_luma_u8.image_type(), (Channels::Luma, BitDepth::_8));
        assert!(img_luma_u8.is_luma());
        assert!(img_luma_u8.as_luma_u8().is_ok());
        let img_luma_u16 = open("./test_data/io/png/grayscale_16bit.png").unwrap();
        assert_eq!(img_luma_u16.image_type(), (Channels::Luma, BitDepth::_16));
        assert!(img_luma_u16.is_luma());
        assert!(img_luma_u16.as_luma_u16().is_ok());
        let img_luma_alpha_u8 = open("./test_data/io/png/grayscale_alpha_8bit.png").unwrap();
        assert_eq!(
            img_luma_alpha_u8.image_type(),
            (Channels::LumaA, BitDepth::_8)
        );
        assert!(img_luma_alpha_u8.is_luma_alpha());
        assert!(img_luma_alpha_u8.as_luma_alpha_u8().is_ok());
        let img_luma_alpha_u16 = open("./test_data/io/png/grayscale_alpha_16bit.png").unwrap();
        assert_eq!(
            img_luma_alpha_u16.image_type(),
            (Channels::LumaA, BitDepth::_16)
        );
        assert!(img_luma_alpha_u16.is_luma_alpha());
        assert!(img_luma_alpha_u16.as_luma_alpha_u16().is_ok());
        let img_rgb_u8 = open("./test_data/io/png/rgb_8bit.png").unwrap();
        assert_eq!(img_rgb_u8.image_type(), (Channels::Rgb, BitDepth::_8));
        assert!(img_rgb_u8.is_rgb());
        assert!(img_rgb_u8.as_rgb_u8().is_ok());
        let img_rgb_u16 = open("./test_data/io/png/rgb_16bit.png").unwrap();
        assert_eq!(img_rgb_u16.image_type(), (Channels::Rgb, BitDepth::_16));
        assert!(img_rgb_u16.is_rgb());
        assert!(img_rgb_u16.as_rgb_u16().is_ok());
        let img_rgb_alpha_u8 = open("./test_data/io/png/rgba_8bit.png").unwrap();
        assert_eq!(
            img_rgb_alpha_u8.image_type(),
            (Channels::RgbA, BitDepth::_8)
        );
        assert!(img_rgb_alpha_u8.is_rgb_alpha());
        assert!(img_rgb_alpha_u8.as_rgb_alpha_u8().is_ok());
        let img_rgb_alpha_u16 = open("./test_data/io/png/rgba_16bit.png").unwrap();
        assert_eq!(
            img_rgb_alpha_u16.image_type(),
            (Channels::RgbA, BitDepth::_16)
        );
        assert!(img_rgb_alpha_u16.is_rgb_alpha());
        assert!(img_rgb_alpha_u16.as_rgb_alpha_u16().is_ok());
    }

    #[test]
    fn test_open_tiff() {
        let img_luma_u8 = open("./test_data/io/tiff/grayscale_8bit.tiff").unwrap();
        assert_eq!(img_luma_u8.image_type(), (Channels::Luma, BitDepth::_8));
        assert!(img_luma_u8.is_luma());
        assert!(img_luma_u8.as_luma_u8().is_ok());
        let img_luma_u16 = open("./test_data/io/tiff/grayscale_16bit.tiff").unwrap();
        assert_eq!(img_luma_u16.image_type(), (Channels::Luma, BitDepth::_16));
        assert!(img_luma_u16.is_luma());
        assert!(img_luma_u16.as_luma_u16().is_ok());
        //let img_luma_alpha_u8 = open("./test_data/io/tiff/grayscale_alpha_8bit.tiff").unwrap();
        //assert_eq!(img_luma_alpha_u8.image_type(), (Channels::LumaA, BitDepth::_8));
        //assert!(img_luma_alpha_u8.is_luma_alpha());
        //assert!(img_luma_alpha_u8.as_luma_alpha_u8().is_ok());
        //let img_luma_alpha_u16 = open("./test_data/io/tiff/grayscale_alpha_16bit.tiff").unwrap();
        //assert_eq!(img_luma_alpha_u16.image_type(), (Channels::LumaA, BitDepth::_16));
        //assert!(img_luma_alpha_u16.is_luma_alpha());
        //assert!(img_luma_alpha_u16.as_luma_alpha_u16().is_ok());
        let img_rgb_u8 = open("./test_data/io/tiff/rgb_8bit.tiff").unwrap();
        assert_eq!(img_rgb_u8.image_type(), (Channels::Rgb, BitDepth::_8));
        assert!(img_rgb_u8.is_rgb());
        assert!(img_rgb_u8.as_rgb_u8().is_ok());
        let img_rgb_u16 = open("./test_data/io/tiff/rgb_16bit.tiff").unwrap();
        assert_eq!(img_rgb_u16.image_type(), (Channels::Rgb, BitDepth::_16));
        assert!(img_rgb_u16.is_rgb());
        assert!(img_rgb_u16.as_rgb_u16().is_ok());
        let img_rgb_alpha_u8 = open("./test_data/io/tiff/rgba_8bit.tiff").unwrap();
        assert_eq!(
            img_rgb_alpha_u8.image_type(),
            (Channels::RgbA, BitDepth::_8)
        );
        assert!(img_rgb_alpha_u8.is_rgb_alpha());
        assert!(img_rgb_alpha_u8.as_rgb_alpha_u8().is_ok());
        let img_rgb_alpha_u16 = open("./test_data/io/tiff/rgba_16bit.tiff").unwrap();
        assert_eq!(
            img_rgb_alpha_u16.image_type(),
            (Channels::RgbA, BitDepth::_16)
        );
        assert!(img_rgb_alpha_u16.is_rgb_alpha());
        assert!(img_rgb_alpha_u16.as_rgb_alpha_u16().is_ok());
    }
}
