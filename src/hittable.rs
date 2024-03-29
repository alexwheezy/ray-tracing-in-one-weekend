use crate::{
    interval::Interval,
    material::MaterialType,
    ray::Ray,
    vec3::{self, Point3, Vec3},
};

use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Arc<MaterialType>>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = vec3::dot(*r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Option<Arc<MaterialType>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: MaterialType) -> Self {
        Self {
            center,
            radius,
            material: Some(Arc::new(material)),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, *r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();

        true
    }
}
