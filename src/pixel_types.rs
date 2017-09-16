use num_traits::Zero;

use traits::{Primitive, Pixel, PixelOps};

use std::ops::{Add, Sub, Mul, Div, Rem};

macro_rules! impl_pixels {
    ( $( $name:ident, $n_channels:expr);+ ) =>
    {$(
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $name<P>
            where P: Primitive
        {
            pub data: [P; $n_channels]
        }

        impl<P> $name<P>
            where P: Primitive
        {
            /// Create a new $name
            pub fn new(data: [P; $n_channels]) -> $name<P> {
                $name { data: data }
            }
        }

        impl<P> Add for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn add(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for i in 0..$n_channels {
                    data[i] = self.data[i] + rhs.data[i];
                }
                $name { data: data }
            }
        }

        impl<P> Sub for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn sub(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for i in 0..$n_channels {
                    data[i] = self.data[i] - rhs.data[i];
                }
                $name { data: data }
            }
        }

        impl<'a, 'b, P> Sub<&'b $name<P>> for &'a $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn sub(self, rhs: &'b $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for i in 0..$n_channels {
                    data[i] = self.data[i] - rhs.data[i];
                }
                $name { data: data }
            }
        }

        impl<P> Mul for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn mul(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for i in 0..$n_channels {
                    data[i] = self.data[i] * rhs.data[i];
                }
                $name { data: data }
            }
        }

        //impl<'a, P> Mul for &'a $name<P>
            //where P: Primitive
        //{
            //type Output = $name<P>;

            //fn mul(&'a self, &'a rhs: $name<P>) -> $name<P> {
                //let mut data = [<P as Zero>::zero(); $n_channels];
                //for i in 0..$n_channels {
                    //data[i] = self.data[i] * rhs.data[i];
                //}
                //$name { data: data }
            //}
        //}

        impl<P> Div for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn div(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for i in 0..$n_channels {
                    data[i] = self.data[i] / rhs.data[i];
                }
                $name { data: data }
            }
        }

        impl<P> Rem for $name<P>
            where P: Primitive
        {
            type Output = $name<P>;

            fn rem(self, rhs: $name<P>) -> $name<P> {
                let mut data = [<P as Zero>::zero(); $n_channels];
                for i in 0..$n_channels {
                    data[i] = self.data[i] % rhs.data[i];
                }
                $name { data: data }
            }
        }

        impl<P> Zero for $name<P>
            where P: Primitive
        {
            fn zero() -> $name<P> {
                $name { data: [<P as Zero>::zero(); $n_channels] }
            }

            fn is_zero(&self) -> bool {
                self.data.iter().map(|p| p.is_zero()).fold(true, |acc, b| b && acc)
            }
        }

        impl<P> From<[P; $n_channels]> for $name<P>
            where P: Primitive
        {
            fn from(array: [P; $n_channels]) -> $name<P> {
                $name { data: array }
            }
        }

        impl<P> Pixel for $name<P>
            where P: Primitive
        {
            type Subpixel = P;

            fn n_channels() -> usize { $n_channels }

            fn channels(&self) -> &[P] { &self.data }

            fn channels_mut(&mut self) -> &mut [P] { &mut self.data }
        }

        impl<P> PixelOps for $name<P>
            where P: Primitive
        { }
    )+}
}

impl_pixels!(
    Luma, 1;
    Rgb, 3
);

impl<P> From<P> for Luma<P>
    where P: Primitive
{
    fn from(data: P) -> Luma<P> {
        Luma { data: [data] }
    }
}
