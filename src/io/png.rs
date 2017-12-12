//! PNG format encoders and decoders.

use image2d::Image2D;
use pixel_types::*;
use traits::Pixel;

use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use failure::Error;
use png::{Decoder, Reader, DecodingError, Encoder, EncodingError, ColorType, BitDepth, HasParameters, Transformations};

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
pub struct PngDecoder<R> where R: Read {
    reader: Reader<R>,
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
        let mut dec = Decoder::new(buffer);
        let trans = Transformations::empty();
        dec.set(trans);
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
            // TODO: what to do for other pixel types ?
            _ => return Err(PngDecodingError::Internal.into())
        };
        Ok(PngDecoder { reader: reader, channels: channels, subpixel: subpixel })
    }

    /// Try reading the image as 8bit grayscale.
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

    /// Try reading the image as 16bit grayscale.
    pub fn read_luma_u16(mut self) -> Result<Image2D<Luma<u16>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::Luma, SubpixelType::U16) => {
                // Read file info and make sure the returned buffer size is coherent with the dimensions
                let (w, h) = self.reader.info().size();
                let buf_size = self.reader.output_buffer_size();
                let expected_size = (w * h * 2) as usize;
                ensure!(buf_size == expected_size, "Invalid buffer size {} (expected {})", buf_size, expected_size);

                // Read the frame into a byte buffer
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));

                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let luma_buffer = u16_buffer.into_iter().map(|i| Luma{ data: [ i as u16 ] }).collect::<Vec<Luma<u16>>>();
                Ok(try!(Image2D::from_vec(w, h, luma_buffer)))
            },
            (_, _) => Err(PngDecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as RGB 8bit.
    pub fn read_rgb_u8(mut self) -> Result<Image2D<Rgb<u8>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::RGB, SubpixelType::U8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                let rgb_buffer = (&buffer).chunks(3)
                                         .map(|s| Rgb { data: [ s[0], s[1], s[2] ] })
                                                 .collect::<Vec<Rgb<u8>>>();
                Ok(try!(Image2D::from_vec(self.reader.info().width, self.reader.info().height, rgb_buffer)))
            },
            (_, _) => Err(PngDecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
        }
    }

    /// Try reading the image as RGB 16bit.
    pub fn read_rgb_u16(mut self) -> Result<Image2D<Rgb<u16>>, Error> {
        match (self.channels, self.subpixel) {
            (ImageChannels::RGB, SubpixelType::U16) => {
                let (w, h) = self.reader.info().size();
                let buf_size = self.reader.output_buffer_size();
                let expected_size = (w * h * 6) as usize;
                ensure!(buf_size == expected_size, "Invalid buffer size {} (expected {})", buf_size, expected_size);

                let mut buffer = vec![ 0; buf_size ];
                try!(self.reader.next_frame(&mut buffer));
                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let rgb_buffer = (&u16_buffer).chunks(3)
                                              .map(|s| Rgb { data: [ s[0], s[1], s[2] ] })
                                              .collect::<Vec<Rgb<u16>>>();
                Ok(try!(Image2D::from_vec(w, h, rgb_buffer)))
            },
            (_, _) => Err(PngDecodingError::IncorrectPixelType(self.channels, self.subpixel).into())
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

/// 8bit PNG encoder type
pub struct PngEncoder8<'a, W, P> where W: Write, P: Pixel<Subpixel=u8> + 'a {
    encoder: Encoder<W>,
    img: &'a Image2D<P>
}

/// 16bit PNG encoder type
pub struct PngEncoder16<'a, W, P> where W: Write, P: Pixel<Subpixel=u16> + 'a {
    encoder: Encoder<W>,
    img: &'a Image2D<P>
}

impl<'a, W, P> PngEncoder8<'a, W, P>
    where W: Write,
          P: Pixel<Subpixel=u8> + 'a
{
    /// Create a new PNG encoder object.
    pub fn new(image: &'a Image2D<P>, out: W) -> Result<PngEncoder8<'a, W, P>, Error>
    {
        let mut enc = Encoder::new(out, image.width(), image.height());
        enc.set(BitDepth::Eight).set(match P::n_channels() {
            1 => ColorType::Grayscale,
            3 => ColorType::RGB,
            _ => return Err(PngEncodingError::UnsupportedType().into())
        });
        Ok(PngEncoder8 { encoder: enc, img: image })
    }

    /// Write to the output buffer.
    pub fn write(self) -> Result<(), Error> {
        // TODO: be more gracious
        let buffer = try!(self.img.as_slice().ok_or(PngEncodingError::Internal));
        let mut u8_buffer = Vec::with_capacity((self.img.width() * self.img.height()) as usize * P::n_channels());
        for pix in buffer {
            u8_buffer.extend_from_slice(pix.channels());
        }
        let mut writer = try!(self.encoder.write_header());
        try!(writer.write_image_data(u8_buffer.as_slice()));
        Ok(())
    }
}

impl<'a, W, P> PngEncoder16<'a, W, P>
    where W: Write,
          P: Pixel<Subpixel=u16> + 'a
{
    /// Create a new PNG encoder object.
    pub fn new(image: &'a Image2D<P>, out: W) -> Result<PngEncoder16<'a, W, P>, Error>
    {
        let mut enc = Encoder::new(out, image.width(), image.height());
        enc.set(BitDepth::Sixteen).set(match P::n_channels() {
            1 => ColorType::Grayscale,
            3 => ColorType::RGB,
            _ => return Err(PngEncodingError::UnsupportedType().into())
        });
        Ok(PngEncoder16 { encoder: enc, img: image })
    }

    /// Write to the output buffer.
    pub fn write(self) -> Result<(), Error> {
        // TODO: be more gracious
        let buffer = try!(self.img.as_slice().ok_or(PngEncodingError::Internal));
        let mut u16_buffer = Vec::with_capacity((self.img.width() * self.img.height()) as usize * P::n_channels());
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
    use super::*;
    use traits::{Pixel, Primitive};

    use num_traits::{NumCast, Zero};

    use std::env::current_dir;
    use std::io::Cursor;
    use std::fmt::Debug;
    use std::fs::File;

    fn mk_test_img<P, S>() -> Image2D<P>
        where P: Pixel<Subpixel=S> + Zero,
              S: Primitive + Sized
    {
        let mut img = Image2D::new(32, 32);
        for y in 0..32 {
            for x in 0..32 {
                let n = <S as NumCast>::from::<u32>(x + y).unwrap();
                let pix = vec![n; P::n_channels()];
                img.put_pixel(x, y, P::from_slice(&pix));
            }
        }
        img
    }

    fn helper_test_read<F, P>(img_path: &'static str, read_fn: F, w: u32, h: u32) -> Result<Image2D<P>, Error>
        where F: FnOnce(PngDecoder<&File>) -> Result<Image2D<P>, Error>,
              P: Pixel,
    {
        let mut test_img = try!(current_dir());
        test_img.push(img_path);
        let file = try!(File::open(test_img));
        let decoder = try!(PngDecoder::new(&file));
        let img = try!(read_fn(decoder));
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);
        Ok(img)
    }

    fn helper_test_write_roundtrip_u8<F, P>(img: Image2D<P>, fn_decode: F)
        where F: FnOnce(PngDecoder<Cursor<&[u8]>>) -> Result<Image2D<P>, Error>,
              P: Pixel<Subpixel=u8> + Debug
    {
        let mut buf = vec![0; 200000];
        {
            let cursor = Cursor::new(buf.as_mut_slice());
            let encoder = PngEncoder8::new(&img, cursor).unwrap();
            encoder.write().unwrap();
        }
        let read_cursor = Cursor::new(buf.as_slice());
        let decoder = PngDecoder::new(read_cursor).unwrap();
        let img2 = fn_decode(decoder).unwrap();
        assert_eq!(img, img2);
    }

    fn helper_test_write_roundtrip_u16<F, P>(img: Image2D<P>, fn_decode: F)
        where F: FnOnce(PngDecoder<Cursor<&[u8]>>) -> Result<Image2D<P>, Error>,
              P: Pixel<Subpixel=u16> + Debug
    {
        let mut buf = vec![0; 200000];
        {
            let cursor = Cursor::new(buf.as_mut_slice());
            let encoder = PngEncoder16::new(&img, cursor).unwrap();
            encoder.write().unwrap();
        }
        let read_cursor = Cursor::new(buf.as_slice());
        let decoder = PngDecoder::new(read_cursor).unwrap();
        let img2 = fn_decode(decoder).unwrap();
        assert_eq!(img, img2);
    }

    #[test]
    fn test_read_luma_u8() {
        helper_test_read("test_data/io/png/grayscale_8bit.png", |d| d.read_luma_u8(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_luma_u16() {
        helper_test_read("test_data/io/png/grayscale_16bit.png", |d| d.read_luma_u16(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_rgb_u8() {
        helper_test_read("test_data/io/png/rgb_8bit.png", |d| d.read_rgb_u8(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_rgb_u16() {
        helper_test_read("test_data/io/png/rgb_16bit.png", |d| d.read_rgb_u16(), 32, 32).unwrap();
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
