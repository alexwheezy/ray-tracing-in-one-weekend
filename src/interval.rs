use crate::rtweekend::INFINITY;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn empty() -> Self {
        Self::new(INFINITY, -INFINITY)
    }

    pub fn universe() -> Self {
        Self::new(-INFINITY, INFINITY)
    }
}
