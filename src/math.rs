use helper::generic::f64_to_float;

use num_traits::Float;

use std::f64::consts::PI;

pub fn gaussian_2d<T>(x: T, y: T, sigma: T) -> T
    where T: Float
{
    let sigma2_2 = f64_to_float::<T>(2.) * sigma * sigma;
    (f64_to_float::<T>(PI) * sigma2_2).recip() * (-(x*x + y*y) / sigma2_2).exp()
}
