use std::f32::INFINITY;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
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
        self.min.max(self.max.min(x))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let interval = Interval::new(1.0, 2.0);
        assert!(interval.contains(1.0));

        assert!(interval.contains(1.5));
        assert!(interval.contains(2.0));

        assert!(!interval.contains(0.0));
        assert!(!interval.contains(3.0));
    }

    #[test]
    fn test_surrounds() {
        let interval = Interval::new(1.0, 2.0);
        assert!(!interval.surrounds(1.0));

        let interval = Interval::new(1.5, 2.0);
        assert!(!interval.surrounds(1.0));
    }

    #[test]
    fn test_clamp() {
        let interval = Interval::new(1.0, 2.0);
        assert_eq!(interval.clamp(0.5), 1.0);
        assert_eq!(interval.clamp(1.5), 1.5);
        assert_eq!(interval.clamp(0.0), 1.0);

        assert_eq!(interval.clamp(3.0), 2.0);
        assert_eq!(interval.clamp(-0.5), 1.0);
    }

    #[test]
    fn test_smoke() {
        assert_eq!(Interval::default().min, -INFINITY);
        assert_eq!(Interval::default().max, INFINITY);

        assert_eq!(Interval::universe().min, -INFINITY);
        assert_eq!(Interval::universe().max, INFINITY);

        assert_eq!(Interval::empty().min, INFINITY);
        assert_eq!(Interval::empty().max, -INFINITY);
    }
}
