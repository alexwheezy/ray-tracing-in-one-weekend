use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::ops::{Index, IndexMut};

use crate::rtweekend;

// Point3 is just an alias for vec3, but useful for geometric clarity in the code.
pub type Point3 = Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn with_vec(vec: &Vec3) -> Self {
        Self { ..*vec }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.z
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn random_vector() -> Vec3 {
        Vec3::new(
            rtweekend::random_double(),
            rtweekend::random_double(),
            rtweekend::random_double(),
        )
    }

    #[inline]
    pub fn random_vector_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            rtweekend::random_double_range(min, max),
            rtweekend::random_double_range(min, max),
            rtweekend::random_double_range(min, max),
        )
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

// Vector Utility Functions

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(
            self[0] * 1.0 / rhs,
            self[1] * 1.0 / rhs,
            self[2] * 1.0 / rhs,
        )
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3::new(
        lhs[1] * rhs[2] - lhs[2] * rhs[1],
        lhs[2] * rhs[0] - lhs[0] * rhs[2],
        lhs[0] * rhs[1] - lhs[1] * rhs[0],
    )
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_vector_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

#[inline]
pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_vector();
    if dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[inline]
pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * dot(v, normal) * normal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_method() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length(), 3.7416575);
        assert_eq!(v.length_squared(), 14.0);
        assert_eq!(unit_vector(&v), v / 3.7416575);
    }

    #[test]
    #[should_panic]
    fn test_invalid_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        panic!("{}", v[3]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_index_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[3] = 4.0;
    }

    #[test]
    fn test_add_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_mul_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_mul_f32() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 / v2, Vec3::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn test_div_f32() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_dot_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(dot(v1, v2), 32.0);

        let v1 = Vec3::new(-1.0, 0.0, -1.0);
        let v2 = Vec3::new(1.0, -2.0, 1.0);
        assert_eq!(dot(v1, v2), -2.0);
    }

    #[test]
    fn test_cross_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(cross(v1, v2), Vec3::new(-3.0, 6.0, -3.0));

        let v1 = Vec3::new(-1.0, 0.0, -1.0);
        let v2 = Vec3::new(1.0, -2.0, 1.0);
        assert_eq!(cross(v1, v2), Vec3::new(-2.0, 0.0, 2.0));
    }
}
