#![allow(clippy::excessive_precision)]

// Constants

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

// Utility Functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
