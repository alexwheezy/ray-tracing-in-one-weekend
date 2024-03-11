use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;
pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let (mut r, mut g, mut b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0,255] value of each color component.
    let intensity = Interval::new(0.0, 0.999);
    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    );
}
