use std::ops;

use rand::Rng;

/// RGB color
///
/// The color values are of the type `f64` and should be in `[0; 1]`.
/// Therefore, (0.0, 0.0, 0.0) is black and (1.0, 1.0, 1.0) is white.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/* CONSTRUCTORS */
impl Color {
    /// Default shorthand constructor
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    /// Constructs a color from `u8` values
    ///
    /// (0, 0, 0) is black; (255, 255, 255) is white.
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        }
    }

    /// Contructs a black color with (0, 0, 0) values
    pub fn black() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Contructs a white color with (255, 255, 255) values
    pub fn white() -> Self {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    /// Constructs a random color with values in `[0; 255]`
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Color {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
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
    /// Color correction (gamma correction, clamp to `[0; 1]`)
    pub fn correct(self, gamma: f64, samples_per_pixel: usize) -> Self {
        let scale = 1.0 / samples_per_pixel as f64;

        let r = (self.r * scale).powf(1.0 / gamma).clamp(0.0, 1.0);
        let g = (self.g * scale).powf(1.0 / gamma).clamp(0.0, 1.0);
        let b = (self.b * scale).powf(1.0 / gamma).clamp(0.0, 1.0);

        Color { r, g, b }
    }

    /// Convert to a brightness value in `[0, 1]`
    pub fn brightness(self) -> f64 {
        let linear = 0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b;
        let srgb = if linear <= 0.0031308 {
            12.92 * linear
        } else {
            1.055 * linear.powf(1.0 / 2.4) - 0.055
        };

        srgb.clamp(0.0, 1.0)
    }
}

/* INFO */
impl Color {
    /// Return `true` if color is close to zero in all values
    pub fn near_zero(&self) -> bool {
        let s = 10_f64.powi(-8);
        (self.r.abs() < s) && (self.g.abs() < s) && (self.b.abs() < s)
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

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

/* TYPE CONVERSION */
impl From<[f64; 3]> for Color {
    fn from(arr: [f64; 3]) -> Self {
        Color {
            r: arr[0],
            g: arr[1],
            b: arr[2],
        }
    }
}

impl From<Color> for [f64; 3] {
    fn from(v: Color) -> Self {
        [v.r, v.g, v.b]
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(t: (f64, f64, f64)) -> Self {
        Color {
            r: t.0,
            g: t.1,
            b: t.2,
        }
    }
}

impl From<Color> for (f64, f64, f64) {
    fn from(v: Color) -> Self {
        (v.r, v.g, v.b)
    }
}

impl From<Color> for crossterm::style::Color {
    fn from(col: Color) -> Self {
        crossterm::style::Color::Rgb {
            r: (col.r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (col.g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (col.b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}
