use ray_tracing_in_one_weekend::{camera, hittable, hittable_list::HittableList, vec3::Point3};

use std::rc::Rc;

fn main() {
    env_logger::init();

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Image
    // Ratio of image width over height
    let aspect_ratio = 16.0 / 9.0;
    // Rendered image width in pixel count
    let image_width = 400;
    // Count of random samples for each pixel
    let sample_per_pixel = 10;

    // Render
    let mut cam = camera::Camera::new(image_width, aspect_ratio, sample_per_pixel);
    cam.render(&world);
}
