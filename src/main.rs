use ray_tracing_in_one_weekend::{
    camera, color, hittable, hittable_list::HittableList, material, vec3::Point3,
};

use std::rc::Rc;

fn main() {
    env_logger::init();

    // World
    let mut world = HittableList::default();

    // Materials
    let material_ground = Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Dielectric::new(1.5));
    let material_left = Rc::new(material::Dielectric::new(1.5));
    let material_right = Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Some(material_ground),
    )));

    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Some(material_center),
    )));

    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(material_left),
    )));

    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right),
    )));

    // Image
    // Ratio of image width over height
    let aspect_ratio = 16.0 / 9.0;
    // Rendered image width in pixel count
    let image_width = 400;
    // Count of random samples for each pixel
    let sample_per_pixel = 30;
    // Maximum number of ray bounces into scene
    let max_depth = 50;

    // Render
    let mut cam = camera::Camera::new(image_width, aspect_ratio, sample_per_pixel, max_depth);
    cam.render(&world);
}
