//! PNG codec.

use core::{
    BitDepth, DynamicImage, Image2D, ImageBuffer2D, ImageType, Luma, LumaA, Pixel, PixelType, Rgb,
    RgbA,
};

use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use failure::Error;

use io::traits::{ImageDecoder, ImageEncoder};
use png;
use png::HasParameters;

use std::io::{Cursor, Read, Write};

/// PNG decoder type
pub struct Decoder<R>
where
    R: Read,
{
    reader: png::Reader<R>,
    channels: PixelType,
    depth: BitDepth,
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
    IncorrectPixelType(PixelType, BitDepth),
    #[fail(display = "Unsupported pixel type: {:?}", _0)]
    /// The image type is not supported (yet) by the library.
    UnsupportedType(png::ColorType),
    #[fail(display = "PNG decoding error")]
    /// Actual decoding error storing the underlying cause.
    Decoder(#[cause] png::DecodingError),
}

impl<R> Decoder<R>
where
    R: Read,
{
    /// Create a new PNG decoder object.
    pub fn new(buffer: R) -> Result<Decoder<R>, Error> {
        let mut dec = png::Decoder::new(buffer);
        let trans = png::Transformations::empty();
        dec.set(trans);
        let (info, reader) = try!(dec.read_info().map_err(DecodingError::Decoder));
        let channels = match info.color_type {
            png::ColorType::Grayscale => PixelType::Luma,
            png::ColorType::GrayscaleAlpha => PixelType::LumaA,
            png::ColorType::RGB => PixelType::Rgb,
            png::ColorType::RGBA => PixelType::RgbA,
            // TODO: support other types
            _ => return Err(DecodingError::UnsupportedType(info.color_type).into()),
        };
        let depth = match info.bit_depth {
            png::BitDepth::Eight => BitDepth::_8,
            png::BitDepth::Sixteen => BitDepth::_16,
            // TODO: what to do for other pixel types ?
            _ => return Err(DecodingError::Internal.into()),
        };
        Ok(Decoder {
            reader,
            channels,
            depth,
        })
    }

    /// Try reading the image as 8bit grayscale.
    pub fn read_luma_u8(mut self) -> Result<ImageBuffer2D<Luma<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Luma, BitDepth::_8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));
                let luma_buffer = buffer
                    .into_iter()
                    .map(|i| Luma { data: [i] })
                    .collect::<Vec<Luma<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    luma_buffer
                )))
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as 8bit grayscale with alpha.
    pub fn read_luma_alpha_u8(mut self) -> Result<ImageBuffer2D<LumaA<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::LumaA, BitDepth::_8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));
                let luma_buffer = (&buffer)
                    .chunks(2)
                    .map(|s| LumaA { data: [s[0], s[1]] })
                    .collect::<Vec<LumaA<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    luma_buffer
                )))
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as 16bit grayscale.
    pub fn read_luma_u16(mut self) -> Result<ImageBuffer2D<Luma<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Luma, BitDepth::_16) => {
                let buf_size = self.reader.output_buffer_size();

                // Read the frame into a byte buffer
                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));

                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let luma_buffer = u16_buffer
                    .into_iter()
                    .map(|i| Luma { data: [i as u16] })
                    .collect::<Vec<Luma<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    luma_buffer
                )))
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as 16bit grayscale with alpha.
    pub fn read_luma_alpha_u16(mut self) -> Result<ImageBuffer2D<LumaA<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::LumaA, BitDepth::_16) => {
                let buf_size = self.reader.output_buffer_size();

                // Read the frame into a byte buffer
                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));

                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let luma_buffer = (&u16_buffer)
                    .chunks(2)
                    .map(|s| LumaA { data: [s[0], s[1]] })
                    .collect::<Vec<LumaA<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    luma_buffer
                )))
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGB 8bit.
    pub fn read_rgb_u8(mut self) -> Result<ImageBuffer2D<Rgb<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Rgb, BitDepth::_8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));
                let rgb_buffer = (&buffer)
                    .chunks(3)
                    .map(|s| Rgb {
                        data: [s[0], s[1], s[2]],
                    })
                    .collect::<Vec<Rgb<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    rgb_buffer
                )))
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGBA 8bit with alpha.
    pub fn read_rgb_alpha_u8(mut self) -> Result<ImageBuffer2D<RgbA<u8>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::RgbA, BitDepth::_8) => {
                let buf_size = self.reader.output_buffer_size();
                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));
                let rgb_buffer = (&buffer)
                    .chunks(4)
                    .map(|s| RgbA {
                        data: [s[0], s[1], s[2], s[3]],
                    })
                    .collect::<Vec<RgbA<u8>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    rgb_buffer
                )))
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGB 16bit.
    pub fn read_rgb_u16(mut self) -> Result<ImageBuffer2D<Rgb<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::Rgb, BitDepth::_16) => {
                let buf_size = self.reader.output_buffer_size();

                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));
                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let rgb_buffer = (&u16_buffer)
                    .chunks(3)
                    .map(|s| Rgb {
                        data: [s[0], s[1], s[2]],
                    })
                    .collect::<Vec<Rgb<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    rgb_buffer
                )))
            }
            (_, _) => Err(DecodingError::IncorrectPixelType(self.channels, self.depth).into()),
        }
    }

    /// Try reading the image as RGB 16bit with alpha.
    pub fn read_rgb_alpha_u16(mut self) -> Result<ImageBuffer2D<RgbA<u16>>, Error> {
        match (self.channels, self.depth) {
            (PixelType::RgbA, BitDepth::_16) => {
                let buf_size = self.reader.output_buffer_size();

                let mut buffer = vec![0; buf_size];
                try!(self.reader.next_frame(&mut buffer));
                // Convert the buffer to a u16 buffer
                let u16_buffer = try!(bytes_to_vec_u16::<BigEndian>(&buffer));
                let rgb_buffer = (&u16_buffer)
                    .chunks(4)
                    .map(|s| RgbA {
                        data: [s[0], s[1], s[2], s[3]],
                    })
                    .collect::<Vec<RgbA<u16>>>();
                Ok(try!(ImageBuffer2D::from_vec(
                    self.reader.info().width,
                    self.reader.info().height,
                    rgb_buffer
                )))
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
    R: Read,
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
            (PixelType::Rgb, BitDepth::_8) => {
                Ok(DynamicImage::RgbU8(Box::new(self.read_rgb_u8()?)))
            }
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

#[derive(Debug, Clone, Default)]
/// 8bit PNG encoder type
pub struct Encoder8;

#[derive(Debug, Clone, Default)]
/// 16bit PNG encoder type
pub struct Encoder16;

impl Encoder8 {
    /// Create a new PNG encoder object.
    pub fn new() -> Encoder8 {
        Encoder8::default()
    }

    /// Write to the output buffer.
    pub fn write<W, P>(&self, out: W, img: &Image2D<P>) -> Result<(), Error>
    where
        W: Write,
        P: Pixel<Subpixel = u8>,
    {
        let (w, h) = img.dimensions();
        let mut enc = png::Encoder::new(out, w, h);
        enc.set(png::BitDepth::Eight).set(match P::N_CHANNELS {
            1 => png::ColorType::Grayscale,
            3 => png::ColorType::RGB,
            _ => return Err(EncodingError::UnsupportedType().into()),
        });
        // TODO: be more gracious
        let buffer = try!(img.as_slice().ok_or(EncodingError::Internal));
        let mut u8_buffer = Vec::with_capacity((w * h * P::N_CHANNELS) as usize);
        for pix in buffer {
            u8_buffer.extend_from_slice(pix.channels());
        }
        let mut writer = try!(enc.write_header());
        try!(writer.write_image_data(u8_buffer.as_slice()));
        Ok(())
    }
}

impl Encoder16 {
    /// Create a new PNG encoder object.
    pub fn new() -> Encoder16 {
        Encoder16::default()
    }

    /// Write to the output buffer.
    pub fn write<W, P>(self, out: W, img: &Image2D<P>) -> Result<(), Error>
    where
        W: Write,
        P: Pixel<Subpixel = u16>,
    {
        let (w, h) = img.dimensions();
        let mut enc = png::Encoder::new(out, w, h);
        enc.set(png::BitDepth::Sixteen).set(match P::N_CHANNELS {
            1 => png::ColorType::Grayscale,
            3 => png::ColorType::RGB,
            _ => return Err(EncodingError::UnsupportedType().into()),
        });
        // TODO: be more gracious
        let buffer = try!(img.as_slice().ok_or(EncodingError::Internal));
        let mut u16_buffer = Vec::with_capacity((w * h * P::N_CHANNELS) as usize);
        for pix in buffer {
            u16_buffer.extend_from_slice(pix.channels());
        }
        let u8_buffer = vec_u16_to_bytes::<BigEndian>(&u16_buffer);
        let mut writer = try!(enc.write_header());
        try!(writer.write_image_data(u8_buffer.as_slice()));
        Ok(())
    }
}

impl<W, P> ImageEncoder<W, P> for Encoder8
where
    W: Write,
    P: Pixel<Subpixel = u8>,
{
    fn write_image(self, out: W, img: &Image2D<P>) -> Result<(), Error> {
        self.write(out, img)
    }
}

impl<W, P> ImageEncoder<W, P> for Encoder16
where
    W: Write,
    P: Pixel<Subpixel = u16>,
{
    fn write_image(self, out: W, img: &Image2D<P>) -> Result<(), Error> {
        self.write(out, img)
    }
}

io_encodable_trait!(
    /// Trait implemented for image types encodable into the PNG format.
    PngEncodable,
    f32;
    f64;
    u32;
    u64;
    i8;
    i16;
    i32;
    i64;
    u8 => {
        |out, img| {
            let enc = Encoder8::new();
            enc.write(out, img)
        }
    };
    u16 => {
        |out, img| {
            let enc = Encoder16::new();
            enc.write(out, img)
        }
    };
);

#[cfg(test)]
mod tests {
    use core::{Image2DMut, ImageBuffer2D, Pixel, Primitive};
    use io::png::*;

    use num_traits::{NumCast, Zero};

    use std::env::current_dir;
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::Cursor;

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

    fn helper_test_read<F, P>(
        img_path: &'static str,
        read_fn: F,
        w: u32,
        h: u32,
    ) -> Result<ImageBuffer2D<P>, Error>
    where
        F: FnOnce(Decoder<&File>) -> Result<ImageBuffer2D<P>, Error>,
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
    where
        F: FnOnce(Decoder<Cursor<&[u8]>>) -> Result<ImageBuffer2D<P>, Error>,
        P: Pixel<Subpixel = u8> + Debug,
    {
        let mut buf = vec![0; 200_000];
        {
            let cursor = Cursor::new(buf.as_mut_slice());
            let encoder = Encoder8::new();
            encoder.write(cursor, &img).unwrap();
        }
        let read_cursor = Cursor::new(buf.as_slice());
        let decoder = Decoder::new(read_cursor).unwrap();
        let img2 = fn_decode(decoder).unwrap();
        assert_eq!(img, img2);
    }

    fn helper_test_write_roundtrip_u16<F, P>(img: ImageBuffer2D<P>, fn_decode: F)
    where
        F: FnOnce(Decoder<Cursor<&[u8]>>) -> Result<ImageBuffer2D<P>, Error>,
        P: Pixel<Subpixel = u16> + Debug,
    {
        let mut buf = vec![0; 200_000];
        {
            let cursor = Cursor::new(buf.as_mut_slice());
            let encoder = Encoder16::new();
            encoder.write(cursor, &img).unwrap();
        }
        let read_cursor = Cursor::new(buf.as_slice());
        let decoder = Decoder::new(read_cursor).unwrap();
        let img2 = fn_decode(decoder).unwrap();
        assert_eq!(img, img2);
    }

    #[test]
    fn test_read_luma_u8() {
        helper_test_read(
            "test_data/io/png/grayscale_8bit.png",
            |d| d.read_luma_u8(),
            32,
            32,
        )
        .unwrap();
    }

    #[test]
    fn test_read_luma_alpha_u8() {
        helper_test_read(
            "test_data/io/png/grayscale_alpha_8bit.png",
            |d| d.read_luma_alpha_u8(),
            32,
            32,
        )
        .unwrap();
    }

    #[test]
    fn test_read_luma_u16() {
        helper_test_read(
            "test_data/io/png/grayscale_16bit.png",
            |d| d.read_luma_u16(),
            32,
            32,
        )
        .unwrap();
    }

    #[test]
    fn test_read_luma_alpha_u16() {
        helper_test_read(
            "test_data/io/png/grayscale_alpha_16bit.png",
            |d| d.read_luma_alpha_u16(),
            32,
            32,
        )
        .unwrap();
    }

    #[test]
    fn test_read_rgb_u8() {
        helper_test_read("test_data/io/png/rgb_8bit.png", |d| d.read_rgb_u8(), 32, 32).unwrap();
    }

    #[test]
    fn test_read_rgb_alpha_u8() {
        helper_test_read(
            "test_data/io/png/rgba_8bit.png",
            |d| d.read_rgb_alpha_u8(),
            32,
            32,
        )
        .unwrap();
    }

    #[test]
    fn test_read_rgb_u16() {
        helper_test_read(
            "test_data/io/png/rgb_16bit.png",
            |d| d.read_rgb_u16(),
            32,
            32,
        )
        .unwrap();
    }

    #[test]
    fn test_read_rgb_alpha_u16() {
        helper_test_read(
            "test_data/io/png/rgba_16bit.png",
            |d| d.read_rgb_alpha_u16(),
            32,
            32,
        )
        .unwrap();
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
