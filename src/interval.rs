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

    #[inline]
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    #[inline]
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    #[inline]
    pub fn clamp(&self, x: f32) -> f32 {
        match x {
            x if x < self.min => self.min,
            x if x > self.max => self.max,
            _ => x,
        }
    }

    #[inline]
    pub fn empty() -> Self {
        Self::new(INFINITY, -INFINITY)
    }

    #[inline]
    pub fn universe() -> Self {
        Self::new(-INFINITY, INFINITY)
    }
}
