//! PNG format encoders and decoders.

use core::{Image2D, ImageBuffer2D, Pixel, Luma, LumaA, Rgb, RgbA};

use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use failure::Error;

use png;
use png::HasParameters;

use std::io::{Read, Write, Cursor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Supported image channel types for PNG I/O.
pub enum ImageChannels {
    /// Grayscale
    Luma,
    /// Grayscale with alpha
    LumaA,
    /// RGB
    RGB,
    /// RGB with alpha
    RGBA
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Supported subpixel types for PNG I/O.
pub enum SubpixelType {
    //I8,
    /// 8bit
    U8,
    //I16,
    /// 16bit
    U16,
}

/// PNG decoder type
pub struct Decoder<R> where R: Read {
    reader: png::Reader<R>,
    channels: ImageChannels,
    subpixel: SubpixelType
}

// Convert a slice of bytes in the specified byte order into a Vec of u16 values.
fn bytes_to_vec_u16<E: ByteOrder>(v: &[u8]) -> Result<Vec<u16>, Error> {
    let size = v.len();
    ensure!(size % 2 == 0, "Vec has odd size");
    let mut v2 = vec![0; size / 2];
    let mut cursor = Cursor::new(v);
    try!(cursor.read_u16_into::<E>(v2.as_mut_slice()));
    Ok(v2)
}

// Convert a slice of u16 values into a Vec of bytes in the specified byte order.
fn vec_u16_to_bytes<E: ByteOrder>(v: &[u16]) -> Vec<u8> {
    let size = v.len();
    let mut v2 = vec![0; size * 2];
    E::write_u16_into(v, v2.as_mut_slice());
    v2
}

#[derive(Fail, Debug)]
/// Represent the errors than can occur when decoding a PNG.
pub enum DecodingError {
    #[fail(display = "Internal decoder error")]
    /// Internal decoder error. These should not actually occur, please report them if you encounter any.
    Internal,
    #[fail(display = "Incorrect pixel type, image type is {:?}({:?})", _0, _1)]
    /// The requested type is not the actual type of the image
    IncorrectPixelType(ImageChannels, SubpixelType),
    #[fail(display = "Unsupported pixel type: {:?}", _0)]
    /// The image type is not supported (yet) by the library.
    UnsupportedType(png::ColorType),
    #[fail(display = "PNG decoding error")]
    /// Actual decoding error storing the underlying cause.
    Decoder(#[cause] png::DecodingError),
}

impl<R> Decoder<R> where R: Read {
    /// Create a new PNG decoder object.
    pub fn new(buffer: R) -> Result<Decoder<R>, Error> {
        let mut dec = png::Decoder::new(buffer);
        let trans = png::Transformations::empty();
        dec.set(trans);
        let (info, reader) = try!(dec.read_info().map_err(DecodingError::Decoder));
        let channels = match info.color_type {
            png::ColorType::Grayscale => ImageChannels::Luma,
            png::ColorType::GrayscaleAlpha => ImageChannels::LumaA,
            png::ColorType::RGB => ImageChannels::RGB,
            png::ColorType::RGBA => ImageChannels::RGBA,
            // TODO: support other types
            _ => return Err(DecodingError::UnsupportedType(info.color_type).into())
        };
        let subpixel = match info.bit_depth {
            png::BitDepth::Eight => SubpixelType::U8,
            png::BitDepth::Sixteen => SubpixelType::U16,
            // TODO: what to do for other pixel types ?
            _ => return Err(DecodingError::Internal.into())
        };
        Ok(Decoder { reader, channels, subpixel })
    }

    /// Try reading the image as 8bit grayscale.
    pub fn read_luma_u8(mut self) -> Result<ImageBuffer2D<Luma<u8>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::Luma, SubpixelType::U8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                let luma_buffer = buffer.into_iter().map(|i| Luma{ data: [ i ] }).collect::<Vec<Luma<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, luma_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as 8bit grayscale with alpha.
    pub fn read_luma_alpha_u8(mut self) -> Result<ImageBuffer2D<LumaA<u8>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::LumaA, SubpixelType::U8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                let luma_buffer = (&buffer).chunks(2).map(|s| LumaA { data: [ s[0], s[1] ] }).collect::<Vec<LumaA<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, luma_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as 16bit grayscale.
    pub fn read_luma_u16(mut self) -> Result<ImageBuffer2D<Luma<u16>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::Luma, SubpixelType::U16) => {
                let buf_size = self.reader.output_buffer_size();

                // Read the frame into a byte buffer
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));

                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let luma_buffer = u16_buffer.into_iter().map(|i| Luma{ data: [ i as u16 ] }).collect::<Vec<Luma<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, luma_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as 16bit grayscale with alpha.
    pub fn read_luma_alpha_u16(mut self) -> Result<ImageBuffer2D<LumaA<u16>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::LumaA, SubpixelType::U16) => {
                let buf_size = self.reader.output_buffer_size();

                // Read the frame into a byte buffer
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));

                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let luma_buffer = (&u16_buffer).chunks(2).map(|s| LumaA { data: [ s[0], s[1] ] }).collect::<Vec<LumaA<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, luma_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as RGB 8bit.
    pub fn read_rgb_u8(mut self) -> Result<ImageBuffer2D<Rgb<u8>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::RGB, SubpixelType::U8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                let rgb_buffer = (&buffer).chunks(3)
                                         .map(|s| Rgb { data: [ s[0], s[1], s[2] ] })
                                                 .collect::<Vec<Rgb<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, rgb_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as RGBA 8bit with alpha.
    pub fn read_rgb_alpha_u8(mut self) -> Result<ImageBuffer2D<RgbA<u8>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::RGBA, SubpixelType::U8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                let rgb_buffer = (&buffer).chunks(4)
                                         .map(|s| RgbA { data: [ s[0], s[1], s[2], s[3] ] })
                                                 .collect::<Vec<RgbA<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, rgb_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as RGB 16bit.
    pub fn read_rgb_u16(mut self) -> Result<ImageBuffer2D<Rgb<u16>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::RGB, SubpixelType::U16) => {
                let buf_size = self.reader.output_buffer_size();

                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let rgb_buffer = (&u16_buffer).chunks(3)
                                              .map(|s| Rgb { data: [ s[0], s[1], s[2] ] })
                                              .collect::<Vec<Rgb<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, rgb_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as RGB 16bit with alpha.
    pub fn read_rgb_alpha_u16(mut self) -> Result<ImageBuffer2D<RgbA<u16>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::RGBA, SubpixelType::U16) => {
                let buf_size = self.reader.output_buffer_size();

                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let rgb_buffer = (&u16_buffer).chunks(4)
                                              .map(|s| RgbA { data: [ s[0], s[1], s[2], s[3] ] })
                                              .collect::<Vec<RgbA<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(self.reader.info().width, self.reader.info().height, rgb_buffer)))
            },
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Return the number of channels in the image.
    pub fn image_channels(&self) -> ImageChannels {
        self.channels
    }

    /// Return the type of the image subpixels.
    pub fn subpixel_type(&self) -> SubpixelType {
        self.subpixel
    }
}

#[derive(Fail, Debug)]
/// Represent the errors than can occur when encoding to a PNG.
pub enum EncodingError {
    #[fail(display = "Internal encoder error")]
    /// Internal encoder error. These should not actually occur, please report them if you encounter any.
    Internal,
    #[fail(display = "Unsupported pixel type")]
    /// The image type is not supported (yet) by the library or by the PNG format.
    UnsupportedType(),
    #[fail(display = "PNG encoding error")]
    /// Actual decoding error storing the underlying cause.
    Encoder(#[cause] png::EncodingError),
}

/// 8bit PNG encoder type
pub struct Encoder8<'a, W, P> where W: Write, P: Pixel<Subpixel=u8> + 'a {
    encoder: png::Encoder<W>,
    img: &'a Image2D<P>
}

/// 16bit PNG encoder type
pub struct Encoder16<'a, W, P> where W: Write, P: Pixel<Subpixel=u16> + 'a {
    encoder: png::Encoder<W>,
    img: &'a Image2D<P>
}

impl<'a, W, P> Encoder8<'a, W, P>
    where W: Write,
          P: Pixel<Subpixel=u8> + 'a
{
    /// Create a new PNG encoder object.
    pub fn new(image: &'a Image2D<P>, out: W) -> Result<Encoder8<'a, W, P>, Error>
    {
        let mut enc = png::Encoder::new(out, image.width(), image.height());
        enc.set(png::BitDepth::Eight).set(match P::N_CHANNELS {
            1 => png::ColorType::Grayscale,
            3 => png::ColorType::RGB,
            _ => return Err(EncodingError::UnsupportedType().into())
        });
        Ok(Encoder8 { encoder: enc, img: image })
    }

    /// Write to the output buffer.
    pub fn write(self) -> Result<(), Error> {
        // TODO: be more gracious
        let buffer = try!(self.img.as_slice().ok_or(EncodingError::Internal));
        let mut u8_buffer = Vec::with_capacity((self.img.width() * self.img.height() * P::N_CHANNELS) as usize);
        for pix in buffer {
            u8_buffer.extend_from_slice(pix.channels());
        }
        let mut writer = try!(self.encoder.write_header());
        try!(writer.write_image_data(u8_buffer.as_slice()));
        Ok(())
    }
}

impl<'a, W, P> Encoder16<'a, W, P>
    where W: Write,
          P: Pixel<Subpixel=u16> + 'a
{
    /// Create a new PNG encoder object.
    pub fn new(image: &'a Image2D<P>, out: W) -> Result<Encoder16<'a, W, P>, Error>
    {
        let mut enc = png::Encoder::new(out, image.width(), image.height());
        enc.set(png::BitDepth::Sixteen).set(match P::N_CHANNELS {
            1 => png::ColorType::Grayscale,
            3 => png::ColorType::RGB,
            _ => return Err(EncodingError::UnsupportedType().into())
        });
        Ok(Encoder16 { encoder: enc, img: image })
    }

    /// Write to the output buffer.
    pub fn write(self) -> Result<(), Error> {
        // TODO: be more gracious
        let buffer = try!(self.img.as_slice().ok_or(EncodingError::Internal));
        let mut u16_buffer = Vec::with_capacity((self.img.width() * self.img.height() * P::N_CHANNELS) as usize);
        for pix in buffer {
            u16_buffer.extend_from_slice(pix.channels());
        }
        let u8_buffer = vec_u16_to_bytes::<BigEndian>(&u16_buffer);
        let mut writer = try!(self.encoder.write_header());
        try!(writer.write_image_data(u8_buffer.as_slice()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::{Image2DMut, ImageBuffer2D, Pixel, Primitive};
    use io::png::*;

    use num_traits::{NumCast, Zero};

    use std::env::current_dir;
    use std::io::Cursor;
    use std::fmt::Debug;
    use std::fs::File;

    fn mk_test_img<P, S>() -> ImageBuffer2D<P>
        where P: Pixel<Subpixel=S> + Zero,
              S: Primitive + Sized
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

    fn helper_test_read<F, P>(img_path: &'static str, read_fn: F, w: u32, h: u32) -> Result<ImageBuffer2D<P>, Error>
        where F: FnOnce(Decoder<&File>) -> Result<ImageBuffer2D<P>, Error>,
              P: Pixel,
    {
        let mut test_img = try!(current_dir());
        test_img.push(img_path);
        let file = try!(File::open(test_img));
        let decoder = try!(Decoder::new(&file));
        let img = try!(read_fn(decoder));
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);
        Ok(img)
    }

    fn helper_test_write_roundtrip_u8<F, P>(img: ImageBuffer2D<P>, fn_decode: F)
        where F: FnOnce(Decoder<Cursor<&[u8]>>) -> Result<ImageBuffer2D<P>, Error>,
              P: Pixel<Subpixel=u8> + Debug
    {
        let mut buf = vec![0; 200000];
        {
            let cursor = Cursor::new(buf.as_mut_slice());
            let encoder = Encoder8::new(&img, cursor).unwrap();
            encoder.write().unwrap();
        }
        let read_cursor = Cursor::new(buf.as_slice());
        let decoder = Decoder::new(read_cursor).unwrap();
        let img2 = fn_decode(decoder).unwrap();
        assert_eq!(img, img2);
    }

    fn helper_test_write_roundtrip_u16<F, P>(img: ImageBuffer2D<P>, fn_decode: F)
        where F: FnOnce(Decoder<Cursor<&[u8]>>) -> Result<ImageBuffer2D<P>, Error>,
              P: Pixel<Subpixel=u16> + Debug
    {
        let mut buf = vec![0; 200000];
        {
            let cursor = Cursor::new(buf.as_mut_slice());
            let encoder = Encoder16::new(&img, cursor).unwrap();
            encoder.write().unwrap();
        }
        let read_cursor = Cursor::new(buf.as_slice());
        let decoder = Decoder::new(read_cursor).unwrap();
        let img2 = fn_decode(decoder).unwrap();
        assert_eq!(img, img2);
    }

    #[test]
    fn test_read_luma_u8() {
        helper_test_read("test_data/io/png/grayscale_8bit.png", |d| d.read_luma_u8(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_luma_alpha_u8() {
        helper_test_read("test_data/io/png/grayscale_alpha_8bit.png", |d| d.read_luma_alpha_u8(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_luma_u16() {
        helper_test_read("test_data/io/png/grayscale_16bit.png", |d| d.read_luma_u16(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_luma_alpha_u16() {
        helper_test_read("test_data/io/png/grayscale_alpha_16bit.png", |d| d.read_luma_alpha_u16(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_rgb_u8() {
        helper_test_read("test_data/io/png/rgb_8bit.png", |d| d.read_rgb_u8(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_rgb_alpha_u8() {
        helper_test_read("test_data/io/png/rgba_8bit.png", |d| d.read_rgb_alpha_u8(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_rgb_u16() {
        helper_test_read("test_data/io/png/rgb_16bit.png", |d| d.read_rgb_u16(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_rgb_alpha_u16() {
        helper_test_read("test_data/io/png/rgba_16bit.png", |d| d.read_rgb_alpha_u16(), 32, 32).unwrap();
    }

    #[test]
    fn test_write_luma_u8() {
        let img = mk_test_img::<Luma<u8>, u8>();
        helper_test_write_roundtrip_u8(img, |d| d.read_luma_u8());
    }

    #[test]
    fn test_write_luma_u16() {
        let img = mk_test_img::<Luma<u16>, u16>();
        helper_test_write_roundtrip_u16(img, |d| d.read_luma_u16());
    }

    #[test]
    fn test_write_rgb_u8() {
        let img = mk_test_img::<Rgb<u8>, u8>();
        helper_test_write_roundtrip_u8(img, |d| d.read_rgb_u8());
    }

    #[test]
    fn test_write_rgb_u16() {
        let img = mk_test_img::<Rgb<u16>, u16>();
        helper_test_write_roundtrip_u16(img, |d| d.read_rgb_u16());
    }
}
