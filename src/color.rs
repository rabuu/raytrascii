use std::ops;

use rand::Rng;

use crate::lalg::Vec3;

/// RGB color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

/* CONSTRUCTORS */
impl Color {
    /// Default shorthand constructor
    pub fn new(r: usize, g: usize, b: usize) -> Self {
        Color { r, g, b }
    }

    /// Contructs a black color with (0, 0, 0) values
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    /// Contructs a white color with (255, 255, 255) values
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    /// Constructs a random color with values in `[0; 255]`
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Color {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

/* CONVERTERS */
impl Color {
    /// Color correction (gamma correction, clamp to `[0; 255]`)
    pub fn correct(self, gamma: f64, samples_per_pixel: usize) -> Self {
        let scale = 1.0 / samples_per_pixel as f64;

        let r = self.r as f64 * scale;
        let g = self.g as f64 * scale;
        let b = self.b as f64 * scale;

        Color {
            r: (r.powf(1.0 / gamma) as usize).clamp(0, 255),
            g: (g.powf(1.0 / gamma) as usize).clamp(0, 255),
            b: (b.powf(1.0 / gamma) as usize).clamp(0, 255),
        }
    }

    /// Convert to a [Vec3] with values in `[0; 1)`
    pub fn to_vec(self) -> Vec3 {
        Vec3 {
            x: self.r as f64 / 255.0,
            y: self.g as f64 / 255.0,
            z: self.b as f64 / 255.0,
        }
    }

    /// Convert to a brightness value in `[0, 1)`
    pub fn brightness(self) -> f64 {
        todo!()
    }
}

/* INFO */
impl Color {
    /// Return `true` if vector is close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;

        let d = 10_f64.powi(-8);
        (r.abs() < d) && (g.abs() < d) && (b.abs() < d)
    }
}

/* ALGEBRAIC OPS */
impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::Mul<usize> for Color {
    type Output = Color;

    fn mul(self, rhs: usize) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::MulAssign<usize> for Color {
    fn mul_assign(&mut self, rhs: usize) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl ops::Mul<Color> for usize {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<usize> for Color {
    type Output = Color;

    fn div(self, rhs: usize) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl ops::DivAssign<usize> for Color {
    fn div_assign(&mut self, rhs: usize) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

/* TYPE CONVERSION */
impl From<[usize; 3]> for Color {
    fn from(arr: [usize; 3]) -> Self {
        Color {
            r: arr[0],
            g: arr[1],
            b: arr[2],
        }
    }
}

impl From<Color> for [usize; 3] {
    fn from(v: Color) -> Self {
        [v.r, v.g, v.b]
    }
}

impl From<(usize, usize, usize)> for Color {
    fn from(t: (usize, usize, usize)) -> Self {
        Color {
            r: t.0,
            g: t.1,
            b: t.2,
        }
    }
}

impl From<Color> for (usize, usize, usize) {
    fn from(v: Color) -> Self {
        (v.r, v.g, v.b)
    }
}
