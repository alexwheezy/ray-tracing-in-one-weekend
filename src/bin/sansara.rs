use ray_tracing_in_one_weekend::{
    camera, color, hittable, hittable_list::HittableList, material, rtweekend, vec3::Point3,
    vec3::Vec3,
};

use std::rc::Rc;

fn generate_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(material::Lambertian::new(color::Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(ground_material),
    )));

    for a in -20..20 {
        for b in -20..20 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                a as f32 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f32 + 0.9 * rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random_vector() * Vec3::random_vector();
                    let sphere_material = Rc::new(material::Lambertian::new(albedo));
                    world.add(Rc::new(hittable::Sphere::new(
                        center,
                        0.2,
                        Some(sphere_material),
                    )));
                } else if choose_mat > 0.6 {
                    // Metal
                    let albedo = Vec3::random_vector_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(material::Metal::new(albedo, fuzz));
                    world.add(Rc::new(hittable::Sphere::new(
                        center,
                        0.2,
                        Some(sphere_material),
                    )));
                } else {
                    // Glass
                    let sphere_material = Rc::new(material::Dielectric::new(1.5));
                    world.add(Rc::new(hittable::Sphere::new(
                        center,
                        0.2,
                        Some(sphere_material),
                    )));
                }
            }
        }
    }

    let material1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Some(material1),
    )));

    let material2 = Rc::new(material::Lambertian::new(color::Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Some(material2),
    )));

    let material3 = Rc::new(material::Metal::new(color::Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(hittable::Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Some(material3),
    )));

    world
}

fn main() {
    env_logger::init();

    // World
    let world = generate_scene();

    // Image
    // Ratio of image width over height
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    // Rendered image width in pixel count
    const IMAGE_WIDTH: i32 = 400;
    // Count of random samples for each pixel
    const SAMPLE_PER_PIXEL: u32 = 50;
    // Maximum number of ray bounces into scene
    const MAX_DEPTH: u32 = 50;

    const VFOV: f32 = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    // Render
    let mut cam = camera::Camera::new(
        IMAGE_WIDTH,
        ASPECT_RATIO,
        SAMPLE_PER_PIXEL,
        MAX_DEPTH,
        VFOV,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_distance,
    );

    cam.render(&world);
}
