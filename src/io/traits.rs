//! Traits related to image I/O.

use core::{DynamicImage, Image2D, ImageBuffer2D, ImageType, Luma, LumaA, Pixel, Rgb, RgbA};

use failure::Error;

use std::io::Write;

/// Trait implemented by all image decoders.
pub trait ImageDecoder: Sized {
    /// Read the image header and return the image information.
    fn read_header(&mut self) -> Result<ImageType, Error>;

    /// Read the image.
    fn read_image(self) -> Result<DynamicImage, Error>;

    /// Try reading the image as 8bit grayscale.
    fn read_luma_u8(self) -> Result<Box<ImageBuffer2D<Luma<u8>>>, Error> {
        self.read_image()?.as_luma_u8()
    }

    /// Try reading the image as 8bit grayscale with alpha.
    fn read_luma_alpha_u8(self) -> Result<Box<ImageBuffer2D<LumaA<u8>>>, Error> {
        self.read_image()?.as_luma_alpha_u8()
    }

    /// Try reading the image as 16bit grayscale.
    fn read_luma_u16(self) -> Result<Box<ImageBuffer2D<Luma<u16>>>, Error> {
        self.read_image()?.as_luma_u16()
    }

    /// Try reading the image as 16bit grayscale with alpha.
    fn read_luma_alpha_u16(self) -> Result<Box<ImageBuffer2D<LumaA<u16>>>, Error> {
        self.read_image()?.as_luma_alpha_u16()
    }

    /// Try reading the image as RGB 8bit.
    fn read_rgb_u8(self) -> Result<Box<ImageBuffer2D<Rgb<u8>>>, Error> {
        self.read_image()?.as_rgb_u8()
    }

    /// Try reading the image as RGBA 8bit with alpha.
    fn read_rgb_alpha_u8(self) -> Result<Box<ImageBuffer2D<RgbA<u8>>>, Error> {
        self.read_image()?.as_rgb_alpha_u8()
    }

    /// Try reading the image as RGB 16bit.
    fn read_rgb_u16(self) -> Result<Box<ImageBuffer2D<Rgb<u16>>>, Error> {
        self.read_image()?.as_rgb_u16()
    }

    /// Try reading the image as RGB 16bit with alpha.
    fn read_rgb_alpha_u16(self) -> Result<Box<ImageBuffer2D<RgbA<u16>>>, Error> {
        self.read_image()?.as_rgb_alpha_u16()
    }
}

/// Trait implemented by all image encoders.
pub trait ImageEncoder<W, P>: Sized
where
    W: Write,
    P: Pixel,
{
    /// Write the image to the output buffer.
    fn write_image(self, out: W, img: &Image2D<P>) -> Result<(), Error>;
}
