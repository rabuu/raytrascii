//! Display and handle brightness values

use std::ops;

const PALETTE: [char; 12] = [' ', '.', ':', ';', '~', '=', 'O', '#', '8', '%', 'B', '@'];

/// Wrapper to hold a brightness value from 0/black to 1/white
#[derive(Debug, Clone, Copy)]
pub struct Brightness(pub f64);

impl Brightness {
    /// Convert brightness value to a corresponding ASCII character
    pub fn to_ascii(self) -> char {
        let b = self.0.clamp(0.0, 1.0);
        let b = b.sqrt(); // gamma 2 correction

        let idx = ((PALETTE.len() - 1) as f64 * (1.0 - b)).round() as usize;
        PALETTE[idx]
    }
}

impl std::fmt::Display for Brightness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ascii())
    }
}

/* ALGEBRAIC OPS */
impl ops::Neg for Brightness {
    type Output = Brightness;

    fn neg(self) -> Self::Output {
        Brightness(-self.0)
    }
}

impl ops::Add<Brightness> for Brightness {
    type Output = Brightness;

    fn add(self, rhs: Brightness) -> Self::Output {
        Brightness(self.0 + rhs.0)
    }
}

impl ops::AddAssign<Brightness> for Brightness {
    fn add_assign(&mut self, rhs: Brightness) {
        self.0 += rhs.0;
    }
}

impl ops::Sub<Brightness> for Brightness {
    type Output = Brightness;

    fn sub(self, rhs: Brightness) -> Self::Output {
        Brightness(self.0 - rhs.0)
    }
}

impl ops::SubAssign<Brightness> for Brightness {
    fn sub_assign(&mut self, rhs: Brightness) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f64> for Brightness {
    type Output = Brightness;

    fn mul(self, rhs: f64) -> Self::Output {
        Brightness(self.0 * rhs)
    }
}

impl ops::MulAssign<f64> for Brightness {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl ops::Mul<Brightness> for f64 {
    type Output = Brightness;

    fn mul(self, rhs: Brightness) -> Self::Output {
        Brightness(rhs.0 * self)
    }
}

impl ops::Div<f64> for Brightness {
    type Output = Brightness;

    fn div(self, rhs: f64) -> Self::Output {
        Brightness(self.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Brightness {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_one() {
        let b = Brightness(1.0);
        assert_eq!(b.to_ascii(), ' ');
    }

    #[test]
    fn white_more_than_one() {
        let b = Brightness(6.9);
        assert_eq!(b.to_ascii(), ' ');
    }

    #[test]
    fn white_slightly_less_than_one() {
        let b = Brightness(0.997);
        assert_eq!(b.to_ascii(), ' ');
    }

    #[test]
    fn dot() {
        let b = Brightness(0.90);
        assert_eq!(b.to_ascii(), '.');
    }

    #[test]
    fn black() {
        let b = Brightness(0.0);
        assert_eq!(b.to_ascii(), '@');
    }
}
