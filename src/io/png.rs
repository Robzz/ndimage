//! Contains the png io code.

use image2d::Image2D;
use pixel_types::*;
use traits::Pixel;

use failure::Error;
use png::{Decoder, Reader, DecodingError, Encoder, EncodingError, ColorType, BitDepth, HasParameters};

use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageChannels {
    Luma,
    LumaA,
    RGB,
    RGBA
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubpixelType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    NonPrimitive
}

/// PNG decoder type
pub struct PngDecoder<R> where R: Read {
    reader: Reader<R>,
    channels: ImageChannels,
    subpixel: SubpixelType
}

#[derive(Fail, Debug)]
/// Represent the errors than can occur when decoding a PNG.
pub enum PngDecodingError {
    #[fail(display = "Internal decoder error")]
    /// Internal decoder error. These should not actually occur, please report them if you encounter any.
    Internal,
    #[fail(display = "Incorrect pixel type, image type is {:?}({:?})", _0, _1)]
    /// The requested type is not the actual type of the image
    IncorrectPixelType(ImageChannels, SubpixelType),
    #[fail(display = "Unsupported pixel type: {:?}", _0)]
    /// The image type is not supported (yet) by the library.
    UnsupportedType(ColorType),
    #[fail(display = "PNG decoding error")]
    /// Actual decoding error storing the underlying cause.
    Decoder(#[cause] DecodingError),
}

impl<R> PngDecoder<R> where R: Read {
    /// Create a new PNG decoder object.
    pub fn new(buffer: R) -> Result<PngDecoder<R>, Error> {
        let dec = Decoder::new(buffer);
        let (info, reader) = try!(dec.read_info().map_err(|e| PngDecodingError::Decoder(e)));
        let channels = match info.color_type {
            ColorType::RGB => ImageChannels::RGB,
            ColorType::Grayscale => ImageChannels::Luma,
            // TODO: support other types
            _ => return Err(PngDecodingError::UnsupportedType(info.color_type).into())
        };
        let subpixel = match info.bit_depth {
            BitDepth::Eight => SubpixelType::U8,
            BitDepth::Sixteen => SubpixelType::U16,
            // png should have expanded lower bit depths to 8
            _ => return Err(PngDecodingError::Internal.into())
        };
        Ok(PngDecoder { reader: reader, channels: channels, subpixel: subpixel })
    }

    pub fn read_luma_u8(mut self) -> Result<Image2D<Luma<u8>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::Luma, SubpixelType::U8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                let luma_buffer = buffer.into_iter().map(|i| Luma{ data: [ i ] }).collect::<Vec<Luma<u8>>>();
                Ok(try!(Image2D::from_vec(self.reader.info().width, self.reader.info().height, luma_buffer)))
            },
            (_, _) => Err(PngDecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    //pub fn read_luma_u16(&self) -> Result<Image2D<Luma<u16>>, Error> {
    //}

    //pub fn read_rgb_u8(&self) -> Result<Image2D<Rgb<u8>>, Error> {
    //}

    //pub fn read_rgb_u16(&self) -> Result<Image2D<Rgb<u16>>, Error> {
    //}

    pub fn image_channels(&self) -> ImageChannels {
        self.channels
    }

    pub fn subpixel_type(&self) -> SubpixelType {
        self.subpixel
    }
}

#[derive(Fail, Debug)]
/// Represent the errors than can occur when encoding to a PNG.
pub enum PngEncodingError {
    #[fail(display = "Internal encoder error")]
    /// Internal encoder error. These should not actually occur, please report them if you encounter any.
    Internal,
    #[fail(display = "Unsupported pixel type")]
    /// The image type is not supported (yet) by the library or by the PNG format.
    UnsupportedType(),
    #[fail(display = "PNG encoding error")]
    /// Actual decoding error storing the underlying cause.
    Encoder(#[cause] EncodingError),
}

/// PNG encoder type
pub struct PngEncoder<'a, W, P> where W: Write, P: Pixel + 'a {
    encoder: Encoder<W>,
    img: &'a Image2D<P>
}

impl<'a, W, P> PngEncoder<'a, W, P> where W: Write, P: Pixel<Subpixel=u8> + 'a {
    /// Create a new PNG encoder object.
    pub fn new(image: &'a Image2D<P>, out: W) -> Result<PngEncoder<'a, W, P>, Error>
    {
        let mut enc = Encoder::new(out, image.width(), image.height());
        enc.set(BitDepth::Eight).set(match P::n_channels() {
            1 => ColorType::Grayscale,
            3 => ColorType::RGB,
            _ => return Err(PngEncodingError::UnsupportedType().into())
        });
        Ok(PngEncoder { encoder: enc, img: image })
    }

    pub fn write(self) -> Result<(), Error> {
        // TODO: be more gracious
        let buffer = try!(self.img.as_slice().ok_or(PngEncodingError::Internal));
        let mut u8_buffer = Vec::with_capacity((self.img.width() * self.img.height()) as usize * P::n_channels());
        for pix in buffer {
            u8_buffer.extend_from_slice(pix.channels());
        }
        let mut writer = try!(self.encoder.write_header());
        try!(writer.write_image_data(&u8_buffer));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;
    use std::io::Cursor;
    use std::fs::File;

    #[test]
    fn test_read_luma_u8() {
        let mut test_img = current_dir().unwrap();
        test_img.push("test_data/io/png");
        test_img.push("grayscale_8bit.png");
        let file = File::open(test_img).unwrap();
        let decoder = PngDecoder::new(&file).unwrap();
        let img = decoder.read_luma_u8().unwrap();
        assert_eq!(img.width(), 32);
        assert_eq!(img.height(), 32);
    }

    #[test]
    fn test_write_luma_u8() {
        let mut test_img = current_dir().unwrap();
        test_img.push("test_data/io/png");
        test_img.push("grayscale_8bit.png");
        let file = File::open(test_img).unwrap();
        let decoder = PngDecoder::new(&file).unwrap();
        let img = decoder.read_luma_u8().unwrap();
        let mut buf = vec![0; 2000];
        {
            let cursor = Cursor::new(buf.as_mut_slice());
            let encoder = PngEncoder::new(&img, cursor).unwrap();
            encoder.write().unwrap();
        }
        let read_cursor = Cursor::new(buf.as_slice());
        let decoder2 = PngDecoder::new(read_cursor).unwrap();
        let img2 = decoder2.read_luma_u8().unwrap();
        assert_eq!(img, img2);
    }
}
