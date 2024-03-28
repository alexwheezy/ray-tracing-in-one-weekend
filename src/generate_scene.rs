use std::sync::Arc;

use crate::color;
use crate::hittable;
use crate::material::{Dielectric, Metal};
use crate::rtweekend;
use crate::vec3::{Point3, Vec3};
use crate::{
    hittable_list::HittableList,
    material::{Lambertian, MaterialType},
};

pub fn random_sphere() -> HittableList {
    let mut world = HittableList::new();

    let ground_material =
        MaterialType::Lambertian(Lambertian::new(color::Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(hittable::Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
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
                    let sphere_material = MaterialType::Lambertian(Lambertian::new(albedo));
                    world.add(Arc::new(hittable::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.9 {
                    // Metal
                    let albedo = Vec3::random_vector_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    let sphere_material = MaterialType::Metal(Metal::new(albedo, fuzz));
                    world.add(Arc::new(hittable::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // Glass
                    let sphere_material = MaterialType::Dielectric(Dielectric::new(1.5));
                    world.add(Arc::new(hittable::Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = MaterialType::Dielectric(Dielectric::new(1.5));
    world.add(Arc::new(hittable::Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = MaterialType::Lambertian(Lambertian::new(color::Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(hittable::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = MaterialType::Metal(Metal::new(color::Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(hittable::Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
