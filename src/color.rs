use float_cmp::approx_eq;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    b: f64,
    g: f64,
}

impl Color {
    pub fn new(r: f64, b: f64, g: f64) -> Self {
        Color { r, b, g }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return approx_eq!(f64, self.r, other.r)
            && approx_eq!(f64, self.b, other.b)
            && approx_eq!(f64, self.g, other.g);
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color::new(self.r + other.r, self.b + other.b, self.g + other.g)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Color::new(self.r - other.r, self.b - other.b, self.g - other.g)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.b * rhs, self.g * rhs)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(self.r * other.r, self.b * other.b, self.g * other.g)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let result = c1 + c2;

        assert_eq!(result, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let result = c1 - c2;

        assert_eq!(result, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);

        assert_eq!(color * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_color_by_another_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let result = c1 * c2;

        assert_eq!(result, Color::new(0.9, 0.2, 0.04));
    }
}
