//! Definition of the dynamic image type.

use core::{BitDepth, ImageBuffer2D, ImageType, Luma, LumaA, PixelType, Rgb, RgbA};

use failure::Error;

/// Image of dynamic pixel type.
pub enum DynamicImage {
    /// 8 bit grayscale image.
    LumaU8(Box<ImageBuffer2D<Luma<u8>>>),
    /// 16 bit grayscale image.
    LumaU16(Box<ImageBuffer2D<Luma<u16>>>),
    /// 8 bit grayscale with alpha image.
    LumaAU8(Box<ImageBuffer2D<LumaA<u8>>>),
    /// 16 bit grayscale with alpha image.
    LumaAU16(Box<ImageBuffer2D<LumaA<u16>>>),
    /// 8 bit color image.
    RgbU8(Box<ImageBuffer2D<Rgb<u8>>>),
    /// 16 bit color image.
    RgbU16(Box<ImageBuffer2D<Rgb<u16>>>),
    /// 8 bit color with alpha image.
    RgbAU8(Box<ImageBuffer2D<RgbA<u8>>>),
    /// 16 bit color with alpha image.
    RgbAU16(Box<ImageBuffer2D<RgbA<u16>>>),
}

impl DynamicImage {
    /// Check whether the image is a grayscale image.
    pub fn is_luma(&self) -> bool {
        match self {
            DynamicImage::LumaU8(_) | DynamicImage::LumaU16(_) => true,
            _ => false,
        }
    }

    /// Check whether the image is a grayscale image with alpha.
    pub fn is_luma_alpha(&self) -> bool {
        match self {
            DynamicImage::LumaAU8(_) | DynamicImage::LumaAU16(_) => true,
            _ => false,
        }
    }

    /// Check whether the image is a color image with alpha.
    pub fn is_rgb(&self) -> bool {
        match self {
            DynamicImage::RgbU8(_) | DynamicImage::RgbU16(_) => true,
            _ => false,
        }
    }

    /// Check whether the image is a color image with alpha.
    pub fn is_rgb_alpha(&self) -> bool {
        match self {
            DynamicImage::RgbAU8(_) | DynamicImage::RgbAU16(_) => true,
            _ => false,
        }
    }

    /// Return the type of the image channels.
    pub fn channels(&self) -> PixelType {
        match self {
            DynamicImage::LumaU8(_) | DynamicImage::LumaU16(_) => PixelType::Luma,
            DynamicImage::LumaAU8(_) | DynamicImage::LumaAU16(_) => PixelType::LumaA,
            DynamicImage::RgbU8(_) | DynamicImage::RgbU16(_) => PixelType::Rgb,
            DynamicImage::RgbAU8(_) | DynamicImage::RgbAU16(_) => PixelType::RgbA,
        }
    }

    /// Return the image bit depth.
    pub fn bit_depth(&self) -> BitDepth {
        match self {
            DynamicImage::LumaU8(_)
            | DynamicImage::LumaAU8(_)
            | DynamicImage::RgbU8(_)
            | DynamicImage::RgbAU8(_) => BitDepth::_8,
            DynamicImage::LumaU16(_)
            | DynamicImage::LumaAU16(_)
            | DynamicImage::RgbU16(_)
            | DynamicImage::RgbAU16(_) => BitDepth::_16,
        }
    }

    /// Return the type of the image.
    pub fn image_type(&self) -> ImageType {
        (self.channels(), self.bit_depth())
    }

    /// Try extracting the image as an 8 bit grayscale image.
    pub fn as_luma_u8(self) -> Result<Box<ImageBuffer2D<Luma<u8>>>, Error> {
        match self {
            DynamicImage::LumaU8(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }

    /// Try extracting the image as a 16 bit grayscale image.
    pub fn as_luma_u16(self) -> Result<Box<ImageBuffer2D<Luma<u16>>>, Error> {
        match self {
            DynamicImage::LumaU16(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }

    /// Try extracting the image as an 8 bit grayscale image with alpha.
    pub fn as_luma_alpha_u8(self) -> Result<Box<ImageBuffer2D<LumaA<u8>>>, Error> {
        match self {
            DynamicImage::LumaAU8(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }

    /// Try extracting the image as a 16 bit grayscale image with alpha.
    pub fn as_luma_alpha_u16(self) -> Result<Box<ImageBuffer2D<LumaA<u16>>>, Error> {
        match self {
            DynamicImage::LumaAU16(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }

    /// Try extracting the image as an 8 bit color image.
    pub fn as_rgb_u8(self) -> Result<Box<ImageBuffer2D<Rgb<u8>>>, Error> {
        match self {
            DynamicImage::RgbU8(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }

    /// Try extracting the image as a 16 bit color image.
    pub fn as_rgb_u16(self) -> Result<Box<ImageBuffer2D<Rgb<u16>>>, Error> {
        match self {
            DynamicImage::RgbU16(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }

    /// Try extracting the image as an 8 bit color image with alpha.
    pub fn as_rgb_alpha_u8(self) -> Result<Box<ImageBuffer2D<RgbA<u8>>>, Error> {
        match self {
            DynamicImage::RgbAU8(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }

    /// Try extracting the image as a 16 bit color image with alpha.
    pub fn as_rgb_alpha_u16(self) -> Result<Box<ImageBuffer2D<RgbA<u16>>>, Error> {
        match self {
            DynamicImage::RgbAU16(img) => Ok(img),
            _ => bail!("Incorrect image type!"),
        }
    }
}
