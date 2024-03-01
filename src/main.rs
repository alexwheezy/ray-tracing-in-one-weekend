use log::info;

use ray_tracing_in_one_weekend::{color, ray, vec3};

fn ray_color(r: &ray::Ray) -> color::Color {
    color::Color::new(1.0, 0.0, 0.0)
}

fn main() {
    env_logger::init();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = {
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        if image_height < 1 {
            1
        } else {
            image_width
        }
    };

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f32;
    let camera_center = vec3::Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = &viewport_u / image_width as f32;
    let pixel_delta_v = &viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center
        - &vec3::Vec3::new(0.0, 0.0, focal_length)
        - &viewport_u / 2.0
        - &viewport_v / 2.0;

    // Render
    println!("P3\n{} {}\n255", image_height, image_width);
    for j in 0..image_height {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = color::Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.0,
            );
            color::write_color(pixel_color);
        }
    }
    info!(" \rDone.                 \n");
}
