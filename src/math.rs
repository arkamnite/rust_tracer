use num::traits::FloatConst;

// Math constants we may need.
const infinity: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::PI() / 180.0
}
