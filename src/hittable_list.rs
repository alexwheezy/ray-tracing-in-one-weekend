use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Option<Rc<dyn Hittable>>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    #[inline]
    pub fn with_object(&mut self, object: Rc<dyn Hittable>) {
        self.add(object);
    }

    #[inline]
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(Some(object));
    }

    #[inline]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.as_ref().unwrap().hit(
                r,
                Interval::new(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
