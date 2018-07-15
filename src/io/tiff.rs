//! TIFF codec.

use core::{BitDepth, PixelType, DynamicImage, ImageBuffer2D, ImageType, Luma, LumaA, Rgb, RgbA};
use io::traits::ImageDecoder;

use failure::Error;

use tiff::{
    decoder::{Decoder as TiffDecoder, DecodingResult},
    ColorType, TiffError,
};

use std::io::{Read, Seek};

/// TIFF decoder type
pub struct Decoder<R>
where
    R: Read + Seek,
{
    reader: TiffDecoder<R>,
    channels: PixelType,
    depth: BitDepth,
    dimensions: (u32, u32),
}

#[derive(Fail, Debug)]
/// Represent the errors than can occur when decoding a TIFF.
pub enum DecodingError {
    #[fail(display = "Internal decoder error")]
    /// Internal decoder error. These should not actually occur, please report them if you encounter any.
    Internal,
    #[fail(
        display = "Incorrect pixel type, image type is {:?}({:?})",
        _0,
        _1
    )]
    /// The requested type is not the actual type of the image
    IncorrectPixelType(PixelType, BitDepth),
    #[fail(display = "Unsupported pixel type: {:?}", _0)]
    /// The image type is not supported (yet) by the library.
    UnsupportedType(ColorType),
    #[fail(display = "TIFF decoding error")]
    /// Actual decoding error storing the underlying cause.
    Decoder(#[cause] TiffError),
}

impl<R> Decoder<R>
where
    R: Read + Seek,
{
    /// Create a new TIFF decoder object.
    pub fn new(buffer: R) -> Result<Decoder<R>, Error> {
        let mut dec = TiffDecoder::new(buffer)?;
        let color_type = dec.colortype()?;
        let (channels, depth) = match &color_type {
            ColorType::Gray(8u8) => (PixelType::Luma, BitDepth::_8),
            ColorType::Gray(16u8) => (PixelType::Luma, BitDepth::_16),
            ColorType::GrayA(8u8) => (PixelType::LumaA, BitDepth::_8),
            ColorType::GrayA(16u8) => (PixelType::LumaA, BitDepth::_16),
            ColorType::RGB(8u8) => (PixelType::Rgb, BitDepth::_8),
            ColorType::RGB(16u8) => (PixelType::Rgb, BitDepth::_16),
            ColorType::RGBA(8u8) => (PixelType::RgbA, BitDepth::_8),
            ColorType::RGBA(16u8) => (PixelType::RgbA, BitDepth::_16),
            // TODO: support other types
            _ => return Err(DecodingError::UnsupportedType(color_type).into()),
        };
        let dimensions = dec.dimensions()?;
        Ok(Decoder {
            reader: dec,
            channels,
            depth,
            dimensions,
        })
    }

    /// Try reading the image as 8bit grayscale.
    pub fn read_luma_u8(mut self) -> Result<ImageBuffer2D<Luma<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Luma, BitDepth::_8) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U8(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let luma_buffer = buffer
                            .into_iter()
                            .map(|i| Luma { data: [i] })
                            .collect::<Vec<Luma<u8>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            luma_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as 8bit grayscale with alpha.
    pub fn read_luma_alpha_u8(mut self) -> Result<ImageBuffer2D<LumaA<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::LumaA, BitDepth::_8) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U8(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1 * 2) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let luma_buffer = buffer
                            .chunks(2)
                            .map(|s| LumaA { data: [s[0], s[1]] })
                            .collect::<Vec<LumaA<u8>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            luma_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as 16bit grayscale.
    pub fn read_luma_u16(mut self) -> Result<ImageBuffer2D<Luma<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Luma, BitDepth::_16) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U16(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let luma_buffer = buffer
                            .into_iter()
                            .map(|i| Luma { data: [i] })
                            .collect::<Vec<Luma<u16>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            luma_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as 16bit grayscale with alpha.
    pub fn read_luma_alpha_u16(mut self) -> Result<ImageBuffer2D<LumaA<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::LumaA, BitDepth::_16) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U16(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1 * 2) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let luma_buffer = buffer
                            .chunks(2)
                            .map(|s| LumaA { data: [s[0], s[1]] })
                            .collect::<Vec<LumaA<u16>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            luma_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGB 8bit.
    pub fn read_rgb_u8(mut self) -> Result<ImageBuffer2D<Rgb<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Rgb, BitDepth::_8) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U8(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1 * 3) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let rgb_buffer = buffer
                            .chunks(3)
                            .map(|s| Rgb {
                                data: [s[0], s[1], s[2]],
                            })
                            .collect::<Vec<Rgb<u8>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            rgb_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGBA 8bit with alpha.
    pub fn read_rgb_alpha_u8(mut self) -> Result<ImageBuffer2D<RgbA<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::RgbA, BitDepth::_8) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U8(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1 * 4) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let rgb_buffer = buffer
                            .chunks(4)
                            .map(|s| RgbA {
                                data: [s[0], s[1], s[2], s[3]],
                            })
                            .collect::<Vec<RgbA<u8>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            rgb_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGB 16bit.
    pub fn read_rgb_u16(mut self) -> Result<ImageBuffer2D<Rgb<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Rgb, BitDepth::_16) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U16(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1 * 3) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let rgb_buffer = buffer
                            .chunks(3)
                            .map(|s| Rgb {
                                data: [s[0], s[1], s[2]],
                            })
                            .collect::<Vec<Rgb<u16>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            rgb_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGB 16bit with alpha.
    pub fn read_rgb_alpha_u16(mut self) -> Result<ImageBuffer2D<RgbA<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::RgbA, BitDepth::_16) => {
                let decoded = self.reader.read_image()?;
                match decoded {
                    DecodingResult::U16(buffer) => {
                        if buffer.len() != (self.dimensions.0 * self.dimensions.1 * 4) as usize {
                            return Err(DecodingError::Internal.into());
                        }
                        let rgb_buffer = buffer
                            .chunks(4)
                            .map(|s| RgbA {
                                data: [s[0], s[1], s[2], s[3]],
                            })
                            .collect::<Vec<RgbA<u16>>>();
                        Ok(try!(ImageBuffer2D::from_vec(
                            self.dimensions.0,
                            self.dimensions.1,
                            rgb_buffer
                        )))
                    }
                    _ => Err(DecodingError::Internal.into()),
                }
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Return the number of channels in the image.
    pub fn image_channels(&self) -> PixelType {
        self.channels
    }

    /// Return the image bit depth.
    pub fn depth(&self) -> BitDepth {
        self.depth
    }
}

impl<R> ImageDecoder for Decoder<R>
where
    R: Read + Seek,
{
    fn read_header(&mut self) -> Result<ImageType, Error> {
        Ok((self.image_channels(), self.depth()))
    }

    fn read_image(mut self) -> Result<DynamicImage, Error> {
        match self.read_header()? {
            (PixelType::Luma, BitDepth::_8) => {
                Ok(DynamicImage::LumaU8(Box::new(self.read_luma_u8()?)))
            }
            (PixelType::Luma, BitDepth::_16) => {
                Ok(DynamicImage::LumaU16(Box::new(self.read_luma_u16()?)))
            }
            (PixelType::LumaA, BitDepth::_8) => {
                Ok(DynamicImage::LumaAU8(Box::new(self.read_luma_alpha_u8()?)))
            }
            (PixelType::LumaA, BitDepth::_16) => Ok(DynamicImage::LumaAU16(Box::new(
                self.read_luma_alpha_u16()?,
            ))),
            (PixelType::Rgb, BitDepth::_8) => Ok(DynamicImage::RgbU8(Box::new(self.read_rgb_u8()?))),
            (PixelType::Rgb, BitDepth::_16) => {
                Ok(DynamicImage::RgbU16(Box::new(self.read_rgb_u16()?)))
            }
            (PixelType::RgbA, BitDepth::_8) => {
                Ok(DynamicImage::RgbAU8(Box::new(self.read_rgb_alpha_u8()?)))
            }
            (PixelType::RgbA, BitDepth::_16) => {
                Ok(DynamicImage::RgbAU16(Box::new(self.read_rgb_alpha_u16()?)))
            }
        }
    }
}
