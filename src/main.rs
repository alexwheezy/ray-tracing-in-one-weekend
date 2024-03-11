#![allow(dead_code)]
#![allow(unused)]

use log::info;
use std::rc::Rc;

use ray_tracing_in_one_weekend::{
    camera::Camera, color, hittable, hittable_list, interval, ray, rtweekend::*, vec3,
};

fn main() {
    env_logger::init();

    // World
    let mut world = hittable_list::HittableList::default();
    world.add(Rc::new(hittable::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(hittable::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Render
    let mut cam = Camera::new(image_width, aspect_ratio);
    cam.render(&world);
}
