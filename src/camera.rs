#![allow(clippy::too_many_arguments)]
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Image {
    pub width: i32,
    pub height: i32,
}

impl Default for Image {
    fn default() -> Self {
        Self::new(640, 480)
    }
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn with_width(width: i32) -> Self {
        const ASPECT_RATIO: f32 = 1.0;
        let height: i32 = (width as f32 / ASPECT_RATIO) as i32;
        Self { width, height }
    }
}

#[derive(Default)]
pub struct Xform {
    pub center: Point3,
    pub pixel00_loc: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Xform {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3) -> Self {
        Self {
            look_from,
            look_at,
            vup,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy)]
pub struct CameraSettings {
    pub aspect_ratio: f32,
    pub vfov: f32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
}

impl CameraSettings {
    pub fn new(aspect_ratio: f32, vfov: f32, defocus_angle: f32, focus_distance: f32) -> Self {
        Self {
            aspect_ratio,
            vfov,
            defocus_angle,
            focus_distance,
        }
    }
}

pub struct Camera {
    pub image: Image,
    pub transform: Xform,
    pub settings: CameraSettings,
}

impl Camera {
    pub fn new(image: Image, transform: Xform, settings: CameraSettings) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = {
            let size = (image.width as f32 / settings.aspect_ratio) as i32;
            if size < 1 {
                1
            } else {
                size
            }
        };
        let image = Image::new(image.width, image_height);
        Self {
            image,
            transform,
            settings,
        }
    }
}
