//! Macros used in several IO modules

/// Generates a helper trait for a specific IO codec, defining a single method allowing to write
/// an image to a buffer. An definition of the trait looks like the following:
///
/// ```
/// trait Example<P>
/// where
///     P: Pixel
/// {
///     fn write_image<W>(out: W, img: &Image2D<P>) -> Result<(), Error>
///     where
///         W: Write;
/// }
/// ```
///
/// This macro generates default implementations for the specified image types, returning an error
/// stating that encoding to this format is not supported for this pixel type. Alternatively, an
/// implementation of the `write_image` function can be provided if the codec supports the type.
/// An invocation of the macro looks like the following:
///
/// ```
/// io_encodable_trait!(
///     /// You can add some documentation to the trait.
///     PngEncodable, // Name of the trait
///     // Now come the image types for which the trait will have the default implementation
///     f32;
///     f64;
///     u32;
///     u6;
///     i8;
///     i16;
///     i32;
///     i64;
///     // But alternatively, an implementation can be provided for a given type.
///     // The macro expects a closure with the same signature as the `write_image` method.
///     u8 => {
///         |out, img| {
///             let enc = io::png::Encoder8::new();
///             enc.write(out, img)
///         }
///     };
///     u16 => {
///         |out, img| {
///             let enc = io::png::Encoder16::new();
///             enc.write(out, img)
///         }
///     };
/// );
/// ```
///
macro_rules! io_encodable_trait {
    ( $(#[$attr:meta])* $name:ident, $($types:tt)+ ) => {
        $( #[$attr] )*
        pub trait $name<P>
        where
            P: Pixel
        {
            /// Try to write the image in the specified format.
            fn write_image<W>(out: W, img: &Image2D<P>) -> Result<(), Error>
            where
                W: Write;
        }

        io_encodable_trait_impls!($name: $($types)+);
    }
}

macro_rules! io_encodable_trait_impls {
    ($name:ident :) => { };
    ($name:ident : $t:ty; $($tail:tt)*) => {
        impl $name<Luma<$t>> for Luma<$t>
        {
            fn write_image<W>(_: W, _: &Image2D<Luma<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                bail!("Image type is not supported for the requested format.")
            }
        }

        impl $name<LumaA<$t>> for LumaA<$t>
        {
            fn write_image<W>(_: W, _: &Image2D<LumaA<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                bail!("Image type is not supported for the requested format.")
            }
        }

        impl $name<Rgb<$t>> for Rgb<$t>
        {
            fn write_image<W>(_: W, _: &Image2D<Rgb<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                bail!("Image type is not supported for the requested format.")
            }
        }

        impl $name<RgbA<$t>> for RgbA<$t>
        {
            fn write_image<W>(_: W, _: &Image2D<RgbA<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                bail!("Image type is not supported for the requested format.")
            }
        }

        io_encodable_trait_impls!($name: $($tail)*);
    };
    ($name:ident : $t:ty => { $c:expr }; $($tail:tt)*) => {
        impl $name<Luma<$t>> for Luma<$t>
        {
            fn write_image<W>(out: W, img: &Image2D<Luma<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                let f: Box<Fn(W, &Image2D<Luma<$t>>) -> Result<(), Error>> = Box::new($c);
                f(out, img)
            }
        }

        impl $name<LumaA<$t>> for LumaA<$t>
        {
            fn write_image<W>(out: W, img: &Image2D<LumaA<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                let f: Box<Fn(W, &Image2D<LumaA<$t>>) -> Result<(), Error>> = Box::new($c);
                f(out, img)
            }
        }

        impl $name<Rgb<$t>> for Rgb<$t>
        {
            fn write_image<W>(out: W, img: &Image2D<Rgb<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                let f: Box<Fn(W, &Image2D<Rgb<$t>>) -> Result<(), Error>> = Box::new($c);
                f(out, img)
            }
        }

        impl $name<RgbA<$t>> for RgbA<$t>
        {
            fn write_image<W>(out: W, img: &Image2D<RgbA<$t>>) -> Result<(), Error>
            where
                W: Write
            {
                let f: Box<Fn(W, &Image2D<RgbA<$t>>) -> Result<(), Error>> = Box::new($c);
                f(out, img)
            }
        }

        io_encodable_trait_impls!($name: $($tail)*);
    }
}
