use std::{ops::{Add, Div, Mul, Sub}, fmt::{Debug, Write}};

#[derive(Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

// TODO implement functions from Criterion B
impl Vec3 {
    pub const LENGTH: usize = 3;
    pub const UP: Vec3 = Vec3(0., 1., 0.);
    pub const ONE: Vec3 = Vec3(1., 1., 1.);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    // TODO add ammendment to Criterion B
    pub fn normalized(&self) -> Vec3 {
        return self.clone() / self.length();
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 
    }

    pub fn length(&self) -> f32 {
        return (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt();
    }

    // Getters
    pub fn retrieve(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.1)
    }

    pub fn x(&self) -> f32 {
        return self.0;
    }

    pub fn y(&self) -> f32 {
        return self.1;
    }

    pub fn z(&self) -> f32 {
        return self.2;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.0 * rhs;
        let y = self.1 * rhs;
        let z = self.2 * rhs;
        return Vec3::new(x, y, z);
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let x = self.0 / rhs;
        let y = self.1 / rhs;
        let z = self.2 / rhs;
        return Vec3::new(x, y, z);
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;
        return Vec3::new(x, y, z);
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;
        return Vec3::new(x, y, z);
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    // preforms the cross product
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.0, self.1, self.2))    
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}
