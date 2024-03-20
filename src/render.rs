use crate::camera::Camera;

use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    rtweekend::{degrees_to_radians, random_double},
    vec3::{self, random_in_unit_disk, Point3, Vec3},
};
use log::info;
use rayon::prelude::*;

pub struct RenderSettings {
    sample_per_pixel: u32,
    max_depth: u32,
}

impl RenderSettings {
    pub fn new(sample_per_pixel: u32, max_depth: u32) -> Self {
        Self {
            sample_per_pixel,
            max_depth,
        }
    }
}

pub struct Render {
    settings: RenderSettings,
    camera: Camera,
}

impl Render {
    pub fn new(settings: RenderSettings, camera: Camera) -> Self {
        Self { settings, camera }
    }

    fn initialize(&mut self) {
        let settings = &self.camera.settings;
        let transform = &mut self.camera.transform;
        let image = &self.camera.image;

        // Camera parameters
        let theta = degrees_to_radians(settings.vfov);
        let h = (theta / 2.0).tan();
        transform.center = transform.look_from;

        // let focal_length = (self.look_from - self.look_at).length();
        // Set the camera position to the origin.
        let viewport_height = 2.0 * h * settings.focus_distance;
        let viewport_width = viewport_height * (image.width / image.height) as f32;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = vec3::unit_vector(transform.look_from - transform.look_at);
        let u = vec3::unit_vector(vec3::cross(transform.vup, w));
        let v = vec3::cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        transform.pixel_delta_u = settings.aspect_ratio * viewport_u / image.width as f32;
        transform.pixel_delta_v = viewport_v / image.height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            transform.center - (settings.focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        transform.pixel00_loc =
            viewport_upper_left + 0.5 * (transform.pixel_delta_u + transform.pixel_delta_v);

        let defocus_radius =
            settings.focus_distance * (degrees_to_radians(settings.defocus_angle) / 2.0).tan();

        transform.defocus_disk_u = u * defocus_radius;
        transform.defocus_disk_v = v * defocus_radius;
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        let image = &self.camera.image;
        info!("Rendering...");
        println!("P3\n{} {}\n255", image.width, image.height);

        for j in 0..image.height {
            info!("Scanlines remaining: {}", image.height - j);
            for i in 0..image.width {
                let pixel_color = (0..self.settings.sample_per_pixel)
                    .into_par_iter()
                    .fold(Color::default, |acc_color, _| {
                        let ray = self.get_ray(i, j);
                        acc_color + Self::ray_color(&ray, self.settings.max_depth, world)
                    })
                    .reduce(Color::default, |acc_color, color| acc_color + color);
                color::write_color(
                    &mut std::io::stdout(),
                    pixel_color,
                    self.settings.sample_per_pixel,
                );
            }
        }
        info!(" \rDone.                 \n");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let transform = &self.camera.transform;
        let settings = &self.camera.settings;

        let pixel_center = transform.pixel00_loc
            + (i as f32 * transform.pixel_delta_u)
            + (j as f32 * transform.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = if settings.defocus_angle <= 0.0 {
            transform.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let transform = &self.camera.transform;
        let p = random_in_unit_disk();
        transform.center + (p[0] * transform.defocus_disk_u) + (p[1] * transform.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let transform = &self.camera.transform;
        let offset = -0.5;
        let px = -offset + random_double();
        let py = -offset + random_double();
        px * transform.pixel_delta_u + py * transform.pixel_delta_v
    }

    fn ray_color(r: &Ray, depth: u32, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::default();
        }

        const EPSILON: f32 = 0.001;
        if world.hit(r, Interval::new(EPSILON, std::f32::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .material
                .as_ref()
                .expect("No material in hit record.")
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::default();
        }

        let unit_direction = vec3::unit_vector(*r.direction());
        let a = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
