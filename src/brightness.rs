const PALETTE: [char; 12] = [' ', '.', ':', ';', '~', '=', 'O', '#', '8', '%', 'B', '@'];

/// Wrapper to hold a brightness value from 0/black to 1/white
#[derive(Debug, Clone, Copy)]
pub struct Brightness(pub f64);

impl Brightness {
    /// Convert brightness value to a corresponding ASCII character
    pub fn to_ascii(self) -> char {
        let b = self.0.clamp(0.0, 1.0);
        let offset = 0.01;

        for i in 0..PALETTE.len() - 1 {
            if (b + offset) >= 1.0 / (i + 1) as f64 {
                return PALETTE[i];
            }
        }

        return *PALETTE.last().unwrap();
    }
}

impl std::fmt::Display for Brightness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ascii())
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
        let b = Brightness(0.97);
        assert_eq!(b.to_ascii(), '.');
    }
}
