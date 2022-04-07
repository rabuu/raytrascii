use std::ops::{self, Range};

use rand::Rng;

/// Three-dimensional vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/* CONSTRUCTORS */
impl Vec3 {
    /// Default shorthand constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Constructs a vector with `V(0, 0, 0)`
    pub fn origin() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    /// Constructs a random vector with values in `[0; 1)`
    pub fn random() -> Self {
        Vec3 {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }

    /// Constructs a random vector with values within a given range
    pub fn random_within_range(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    /// Constructs a random vector that is in the unit sphere
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_within_range(-1.0..1.0);
            if p.len_sq() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    /// Constructs a random unit vector
    pub fn random_unit_vec() -> Self {
        Vec3::random_in_unit_sphere().unit_vec()
    }

    /// Constructs a random vector that is in a hemisphere
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Constructs a random vector that is in the unit disk
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3 {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
                z: 0.0,
            };

            if p.len_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::origin()
    }
}

/* CONVERTERS */
impl Vec3 {
    /// Convert to unit vector
    pub fn unit_vec(self) -> Vec3 {
        self / self.len()
    }
}

/* INFO */
impl Vec3 {
    /// Return squared length
    pub fn len_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Return length
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    /// Return `true` if vector is close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let b = 10_f64.powi(-8);
        (self.x.abs() < b) && (self.y.abs() < b) && (self.z.abs() < b)
    }
}

/* ALGEBRAIC OPS */
impl Vec3 {
    /// Return the dot product/scalar product of two vectors
    pub fn dot(self, rhs: Vec3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Return the cross product/vector product of two vectors
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

/* TYPE CONVERSION */
impl From<[f64; 3]> for Vec3 {
    fn from(arr: [f64; 3]) -> Self {
        Vec3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl From<Vec3> for [f64; 3] {
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(t: (f64, f64, f64)) -> Self {
        Vec3 {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}
