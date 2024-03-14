#![allow(clippy::excessive_precision)]

// Constants

use rand::Rng;

// Utility Functions

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

#[inline]
pub fn random_double() -> f32 {
    rand::thread_rng().gen_range(0.0..1.0)
}

#[inline]
pub fn random_double_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_degrees_to_radians() {
        let degrees: [f32; 10] = [
            12.3, 57.3, 40.2, 24.2, 78.6, -12.3, -57.3, -40.2, -24.2, -78.6,
        ];
        let expected: [f32; 10] = [
            0.2146755,
            1.0000737,
            0.70162237,
            0.42236972,
            1.3718288,
            -0.2146755,
            -1.0000737,
            -0.70162237,
            -0.42236972,
            -1.3718288,
        ];
        for (degree, expect) in degrees.into_iter().zip(expected.into_iter()) {
            assert_eq!(degrees_to_radians(degree), expect);
        }
    }
}
