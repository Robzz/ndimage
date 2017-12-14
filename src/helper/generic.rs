use num_traits::{Float, NumCast};

pub fn f64_to_float<T>(f: f64) -> T
    where T: Float
{
    <T as NumCast>::from::<f64>(f).unwrap()
}
