#![allow(dead_code)]
#![allow(unused)]

use log::info;

use ray_tracing_in_one_weekend::{color, ray, vec3};

fn hit_sphere(center: &vec3::Point3, radius: f32, r: &ray::Ray) -> f32 {
    let oc = *r.origin() - *center;
    let a = vec3::dot(r.direction(), r.direction());
    let b = 2.0 * vec3::dot(&oc, r.direction());
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn ray_color(r: &ray::Ray) -> color::Color {
    let t = hit_sphere(&vec3::Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        info!("hit sphere");
        let n = vec3::unit_vector(&(r.at(t) - vec3::Point3::new(0.0, 0.0, -1.0)));
        return 0.5 * color::Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * color::Color::new(1.0, 1.0, 1.0) + a * color::Color::new(0.5, 0.7, 1.0)
}

fn main() {
    env_logger::init();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = {
        let size = (image_width as f32 / aspect_ratio) as i32;
        if size < 1 {
            1
        } else {
            size
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
    let pixel_delta_u = aspect_ratio * viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    // FIX: Why is the sphere off-cente?
    let viewport_upper_left = camera_center
        - vec3::Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = ray::Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(&ray);
            color::write_color(pixel_color);
        }
    }
    info!(" \rDone.                 \n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_sphere() {
        let center = vec3::Point3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        let ray = ray::Ray::new(
            &vec3::Point3::new(0.0, 0.0, 0.0),
            &vec3::Vec3::new(0.0, 0.0, 1.0),
        );
        assert_eq!(hit_sphere(&center, radius, &ray), -2.5);
    }
}
