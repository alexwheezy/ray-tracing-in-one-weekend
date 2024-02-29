use crate::vec3::Vec3;

pub type Color = Vec3;
pub fn write_color(color: Color) {
    // Write the translated [0,255] value of each color component.
    println!(
        "{} {} {}",
        (255.99 * color.x()) as i32,
        (255.99 * color.y()) as i32,
        (255.99 * color.z()) as i32
    );
}
