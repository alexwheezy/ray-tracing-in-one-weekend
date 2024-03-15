#![allow(clippy::too_many_arguments)]

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

pub struct Camera {
    image_width: i32,
    aspect_ratio: f32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    sample_per_pixel: u32,
    max_depth: u32,
    vfov: f32,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    defocus_angle: f32,
    focus_distance: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f32 = 1.0;
        const IMAGE_WIDTH: i32 = 100;
        const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
        const SAMPLE_PER_PIXEL: u32 = 3;
        const MAX_DEPTH: u32 = 10;
        const VFOV: f32 = 90.0;
        let look_from: Point3 = Point3::new(0.0, 0.0, -1.0);
        let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
        let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        Self {
            image_width: IMAGE_WIDTH,
            aspect_ratio: ASPECT_RATIO,
            image_height: IMAGE_HEIGHT,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            sample_per_pixel: SAMPLE_PER_PIXEL,
            max_depth: MAX_DEPTH,
            vfov: VFOV,
            look_from,
            look_at,
            vup,
            defocus_angle: 0.0,
            focus_distance: 10.0,
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    #[must_use]
    pub fn new(
        image_width: i32,
        aspect_ratio: f32,
        sample_per_pixel: u32,
        max_depth: u32,
        vfov: f32,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f32,
        focal_distance: f32,
    ) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = {
            let size = (image_width as f32 / aspect_ratio) as i32;
            if size < 1 {
                1
            } else {
                size
            }
        };
        Self {
            image_width,
            image_height,
            aspect_ratio,
            sample_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_distance: focal_distance,
            ..Default::default()
        }
    }
    fn initialize(&mut self) {
        // Camera parameters
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        self.center = self.look_from;

        // let focal_length = (self.look_from - self.look_at).length();
        // Set the camera position to the origin.
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * (self.image_width / self.image_height) as f32;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = vec3::unit_vector(self.look_from - self.look_at);
        let u = vec3::unit_vector(vec3::cross(self.vup, w));
        let v = vec3::cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = self.aspect_ratio * viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius =
            self.focus_distance * (degrees_to_radians(self.defocus_angle) / 2.0).tan();

        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        info!("Rendering...");
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }
                color::write_color(&mut std::io::stdout(), pixel_color, self.sample_per_pixel);
            }
        }
        info!(" \rDone.                 \n");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let offset = -0.5;
        let px = -offset + random_double();
        let py = -offset + random_double();
        px * self.pixel_delta_u + py * self.pixel_delta_v
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
