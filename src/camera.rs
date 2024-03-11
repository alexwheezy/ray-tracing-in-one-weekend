use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::INFINITY,
    vec3::{self, Point3, Vec3},
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
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio: f32 = 1.0;
        let image_width = 100;
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        Self {
            image_width,
            aspect_ratio,
            image_height,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }
}

impl Camera {
    #[must_use]
    pub fn new(image_width: i32, aspect_ratio: f32) -> Self {
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
            ..Default::default()
        }
    }
    fn initialize(&mut self) {
        // Camera parameters
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width = viewport_height * (self.image_width / self.image_height) as f32;
        self.center = vec3::Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vec3::Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = self.aspect_ratio * viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        // FIX: Why is the sphere off-cente?
        let viewport_upper_left = self.center
            - vec3::Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        info!("Rendering...");
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(&self.center, &ray_direction);
                let pixel_color = Self::ray_color(&ray, world);
                color::write_color(pixel_color);
            }
        }
        info!(" \rDone.                 \n");
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
