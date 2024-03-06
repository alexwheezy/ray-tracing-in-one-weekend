use crate::{ray::Ray, vec3::Point3, vec3::Vec3};

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> bool;
}

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
}

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> bool {
        todo!()
    }
}
