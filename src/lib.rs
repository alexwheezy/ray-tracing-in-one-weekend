use std::fmt::Display;
use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Neg, Sub};
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

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}
