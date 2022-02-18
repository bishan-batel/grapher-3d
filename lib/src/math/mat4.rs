use core::fmt;
use std::ops::{self, Add, Mul, Sub};

use js_sys::Math;

use super::vec3::Vec3;

pub const MAT4_BUFF_LENGTH: usize = 16;
pub const MAT4_SIDE_LENGTH: usize = 4;

#[derive(Debug)]
pub struct Transform {
    pub proj: Mat4,
    pub view: Mat4,
    pub model: Mat4,
}

/// 4x4 Matrix
#[derive(Clone, Copy)]
pub struct Mat4(pub [f32; MAT4_BUFF_LENGTH]);

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4([
        1., 0., 0., 0., // R1
        0., 1., 0., 0., // R2
        0., 0., 1., 0., // R3
        0., 0., 0., 1., // R4
    ]);

    /// Constructor for empty mat4
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn fill(&mut self, buff: &[f32; MAT4_BUFF_LENGTH]) {
        for i in 0..MAT4_BUFF_LENGTH {
            self.0[i] = buff[i];
        }
    }

    /// Sets single cell in matrix
    pub fn set(&mut self, row: usize, column: usize, val: f32) {
        self.0[row * MAT4_SIDE_LENGTH + column] = val;
    }

    /// Retries single cell in matrix
    pub fn val(&self, row: usize, column: usize) -> f32 {
        self.0[row * MAT4_SIDE_LENGTH + column]
    }

    /// Fills matrix with identity values
    pub fn identity(&mut self) {
        self.fill(&Mat4::IDENTITY.0);
    }

    /// Translates matrix by given vector
    pub fn translate(&mut self, by: &Vec3) {
        let (x, y, z) = by.retrieve();
        self.0[12] = self.0[0] * x + self.0[4] * y + self.0[8] * z + self.0[12];
        self.0[13] = self.0[1] * x + self.0[5] * y + self.0[9] * z + self.0[13];
        self.0[14] = self.0[2] * x + self.0[6] * y + self.0[10] * z + self.0[14];
        self.0[15] = self.0[3] * x + self.0[7] * y + self.0[11] * z + self.0[15];
    }

    pub fn rotate_x(&mut self, by: f32) {
        // creates rotator matrix
        let mut rotator = Mat4::IDENTITY.clone();
        rotator.set(1, 1, by.cos());
        rotator.set(2, 2, by.cos());
        rotator.set(2, 1, -by.sin());
        rotator.set(1, 2, by.sin());

        // applies to transformation to self
        let rotated = rotator * (*self);

        // fill transformed matrix into self
        self.fill(&rotated.0)
    }

    pub fn rotate_y(&mut self, by: f32) {
        // creates rotator matrix
        let mut rotator = Self::IDENTITY.clone();
        rotator.set(0, 0, by.cos());
        rotator.set(2, 2, by.cos());
        rotator.set(0, 2, -by.sin());
        rotator.set(2, 0, by.sin());

        // applies to transformation to self
        let rotated = rotator * (*self);

        // fill transformed matrix into self
        self.fill(&rotated.0)
    }

    pub fn rotate_z(&mut self, by: f32) {
        // creates rotator matrix
        let mut rotator = Self::IDENTITY.clone();
        rotator.set(0, 0, by.cos());
        rotator.set(1, 1, by.cos());
        rotator.set(1, 0, -by.sin());
        rotator.set(0, 1, by.sin());

        // applies to transformation to self
        let rotated = rotator * (*self);

        // fill transformed matrix into self
        self.fill(&rotated.0)
    }

    /// Scales a matrix by given vectory
    pub fn scale(&mut self, by: &Vec3) {
        let (x, y, z) = by.retrieve();
        self.0[0] *= x;
        self.0[1] *= x;
        self.0[2] *= x;
        self.0[3] *= x;
        self.0[4] *= y;
        self.0[5] *= y;
        self.0[6] *= y;
        self.0[7] *= y;
        self.0[8] *= z;
        self.0[9] *= z;
        self.0[10] *= z;
        self.0[11] *= z;
    }

    /// Matrix for creating a perspective projection matrix / frustum
    pub fn perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        let f = 1.0 / (fov / 2.).tan();

        self.0[0] = f / aspect;
        self.0[1] = 0.;
        self.0[2] = 0.;
        self.0[3] = 0.;
        self.0[4] = 0.;
        self.0[5] = f;
        self.0[6] = 0.;
        self.0[7] = 0.;
        self.0[8] = 0.;
        self.0[9] = 0.;
        self.0[11] = -1.;
        self.0[12] = 0.;
        self.0[13] = 0.;
        self.0[15] = 0.;

        if far != f32::INFINITY {
            let near_far = 1. / (near - far);
            self.0[10] = (far + near) * near_far;
            self.0[14] = 2. * far * near * near_far;
        } else {
            self.0[10] = -1.;
            self.0[14] = -2. * near;
        }
    }

    // TODO ammend to B
    /// Calculates the look at matrix from the eye to center, used for
    /// creating View Matricies
    pub fn look_at(&mut self, eye: Vec3, center: Vec3, up: Vec3) {
        // if distance between center and eye is less than minimum
        // value then treat lookAt matrix as identity
        if (eye - center).length() < f32::EPSILON {
            self.identity();
            return;
        }

        // axises
        let z = (eye - center).normalized();
        let x = (up * z).normalized();
        let y = (z * x).normalized();

        self.0 = [
            x.0,
            y.0,
            z.0,
            0.,
            x.1,
            y.1,
            z.1,
            0.,
            x.2,
            y.2,
            z.2,
            0.,
            -x.dot(eye),
            -y.dot(eye),
            -z.dot(eye),
            1.,
        ];
    }
}

// Operator Overloads for mat4 and mat4

impl Add for Mat4 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..MAT4_BUFF_LENGTH {
            self.0[i] += other.0[i];
        }
        self
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        for i in 0..MAT4_BUFF_LENGTH {
            self.0[i] -= other.0[i];
        }
        self
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        // hard coded in favour of loops for efficiency
        Mat4([
            // Row 0 Column 0
            self.0[0] * other.0[0]
                + self.0[1] * other.0[4]
                + self.0[2] * other.0[8]
                + self.0[3] * other.0[12],
            // Row 0 Column 1
            self.0[0] * other.0[1]
                + self.0[1] * other.0[5]
                + self.0[2] * other.0[9]
                + self.0[3] * other.0[13],
            // Row 0 Column 2
            self.0[0] * other.0[2]
                + self.0[1] * other.0[6]
                + self.0[2] * other.0[10]
                + self.0[3] * other.0[14],
            // Row 0 Column 3
            self.0[0] * other.0[3]
                + self.0[1] * other.0[7]
                + self.0[2] * other.0[11]
                + self.0[3] * other.0[15],
            // Row 1 Column 0
            self.0[4] * other.0[0]
                + self.0[5] * other.0[4]
                + self.0[6] * other.0[8]
                + self.0[7] * other.0[12],
            // Row 1 Column 1
            self.0[4] * other.0[1]
                + self.0[5] * other.0[5]
                + self.0[6] * other.0[9]
                + self.0[7] * other.0[13],
            self.0[4] * other.0[2]
                + self.0[5] * other.0[6]
                + self.0[6] * other.0[10]
                + self.0[7] * other.0[14],
            // Row 1 Column 2
            self.0[4] * other.0[3]
                + self.0[5] * other.0[7]
                + self.0[6] * other.0[11]
                + self.0[7] * other.0[15],
            // Row 1 Column 3
            self.0[8] * other.0[0]
                + self.0[9] * other.0[4]
                + self.0[10] * other.0[8]
                + self.0[11] * other.0[12],
            // Row 2 Column 0
            self.0[8] * other.0[1]
                + self.0[9] * other.0[5]
                + self.0[10] * other.0[9]
                + self.0[11] * other.0[13],
            // Row 2 Column 1
            self.0[8] * other.0[2]
                + self.0[9] * other.0[6]
                + self.0[10] * other.0[10]
                + self.0[11] * other.0[14],
            // Row 2 Column 2
            self.0[8] * other.0[3]
                + self.0[9] * other.0[7]
                + self.0[10] * other.0[11]
                + self.0[11] * other.0[15],
            // Row 2 Column 3
            self.0[12] * other.0[0]
                + self.0[13] * other.0[4]
                + self.0[14] * other.0[8]
                + self.0[15] * other.0[12],
            // Row 3 Column 0
            self.0[12] * other.0[1]
                + self.0[13] * other.0[5]
                + self.0[14] * other.0[9]
                + self.0[15] * other.0[13],
            // Row 3 Column 1
            self.0[12] * other.0[2]
                + self.0[13] * other.0[6]
                + self.0[14] * other.0[10]
                + self.0[15] * other.0[14],
            // Row 3 Column 2
            self.0[12] * other.0[3]
                + self.0[13] * other.0[7]
                + self.0[14] * other.0[11]
                + self.0[15] * other.0[15],
        ])
    }
}

// Debug print
impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // hardcoded for slightly efficiency
        f.write_fmt(format_args!(
            "[{} {} {} {}]\n[{} {} {} {}]\n[{} {} {} {}]\n[{} {} {} {}]\n",
            self.val(0, 0),
            self.val(0, 1),
            self.val(0, 2),
            self.val(0, 3),
            self.val(1, 0),
            self.val(1, 1),
            self.val(1, 2),
            self.val(1, 3),
            self.val(2, 0),
            self.val(2, 1),
            self.val(2, 2),
            self.val(2, 3),
            self.val(3, 0),
            self.val(3, 1),
            self.val(3, 2),
            self.val(3, 3),
        ))
    }
}
