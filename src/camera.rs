use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    rtweekend::random_double,
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
    sample_per_pixel: u32,
    max_depth: u32,
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f32 = 1.0;
        const IMAGE_WIDTH: i32 = 100;
        const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
        const SAMPLE_PER_PIXEL: u32 = 3;
        const MAX_DEPTH: u32 = 10;
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
        }
    }
}

impl Camera {
    #[must_use]
    pub fn new(image_width: i32, aspect_ratio: f32, sample_per_pixel: u32, max_depth: u32) -> Self {
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
            ..Default::default()
        }
    }
    fn initialize(&mut self) {
        // Camera parameters
        const FOCAL_LENGTH: f32 = 1.0;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        let viewport_width = VIEWPORT_HEIGHT * (self.image_width / self.image_height) as f32;
        self.center = vec3::Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vec3::Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = self.aspect_ratio * viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center
            - vec3::Vec3::new(self.aspect_ratio / 2.5, 0.0, FOCAL_LENGTH)
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
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
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
