#![allow(unused_variables)]

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::vec3;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub struct Lambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

pub struct Dielectric {
    // Index of Refraction
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    pub fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for MaterialType {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialType::Lambertian(material) => {
                let mut scatter_direction = rec.normal + vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = material.albedo;
                true
            }

            MaterialType::Metal(material) => {
                let reflected = vec3::reflect(vec3::unit_vector(*r_in.direction()), rec.normal);
                *scattered = Ray::new(
                    rec.p,
                    reflected + material.fuzz * vec3::random_unit_vector(),
                );
                *attenuation = material.albedo;
                // vec3::dot(*scattered.direction(), rec.normal) >= 0.0
                true
            }

            MaterialType::Dielectric(material) => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face {
                    1.0 / material.ir
                } else {
                    material.ir
                };
                let unit_direction = vec3::unit_vector(*r_in.direction());
                let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let direction = if refraction_ratio * sin_theta > 1.0
                    || material.reflectance(cos_theta, refraction_ratio) > random_double()
                {
                    vec3::reflect(unit_direction, rec.normal)
                } else {
                    vec3::refract(unit_direction, rec.normal, refraction_ratio)
                };

                *scattered = Ray::new(rec.p, direction);
                true
            }
        }
    }
}
