use crate::prelude::close_enough;
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

    pub fn r(&self) -> &f64 {
        &self.r
    }

    pub fn b(&self) -> &f64 {
        &self.b
    }

    pub fn g(&self) -> &f64 {
        &self.g
    }
}

pub const RED: Color = Color {
    r: 1f64,
    g: 0f64,
    b: 0f64,
};

pub const GREEN: Color = Color {
    r: 0f64,
    g: 1f64,
    b: 0f64,
};

pub const BLUE: Color = Color {
    r: 0f64,
    g: 0f64,
    b: 1f64,
};

pub const WHITE: Color = Color {
    r: 1f64,
    b: 1f64,
    g: 1f64,
};

pub const BLACK: Color = Color {
    r: 0f64,
    b: 0f64,
    g: 0f64,
};

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        close_enough(&self.r, &other.r)
            && close_enough(&self.b, &other.b)
            && close_enough(&self.g, &other.g)
    }
}

impl Eq for Color {}

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
