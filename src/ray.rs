use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self {
            orig: *orig,
            dir: *dir,
        }
    }

    #[inline]
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    #[inline]
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    #[inline]
    pub fn at(&self, t: f32) -> Point3 {
        let point = self.orig + self.dir * t;
        Point3::with_vec(&point)
    }
}
