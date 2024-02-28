use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::ops::{Index, IndexMut};

// Point3 is just an alias for vec3, but useful for geometric clarity in the code.
pub type Point3 = Vec3;

#[derive(Default, Clone)]
pub struct Vec3 {
    data: Vec<f32>,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: vec![x, y, z],
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self[2]
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.data.iter().fold(0.0, |acc, x| acc + x * x)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            data: vec![-self[0], -self[1], -self[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.data.iter_mut().for_each(|x| *x *= rhs);
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.data.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl Index<usize> for &Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

// Vector Utility Functions

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl<'a> Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &'a Vec3) -> Self::Output {
        Self::Output::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<'a> Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &'a Vec3) -> Self::Output {
        Self::Output::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<'a> Mul<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &'a Vec3) -> Self::Output {
        Self::Output::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<'a> Div<f32> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(
            self[0] * 1.0 / rhs,
            self[1] * 1.0 / rhs,
            self[2] * 1.0 / rhs,
        )
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(
        lhs[1] * rhs[2] - lhs[2] * rhs[1],
        lhs[2] * rhs[0] - lhs[0] * rhs[2],
        lhs[0] * rhs[1] - lhs[1] * rhs[0],
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}
