use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GradientPattern {
    start: Color,
    end: Color,
    transform: Transform,
}

impl GradientPattern {
    pub fn new(start: Color, end: Color) -> Self {
        GradientPattern {
            start,
            end,
            transform: Transform::new(IDENTITY_MATRIX),
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl Pattern for GradientPattern {
    fn color_at(&self, point: &Point) -> Color {
        let point_in_pattern_space = self.transform.invert().unwrap() * *point;
        let point_x = point_in_pattern_space.x();
        let distance_from_beginning = point_x + 0.5;

        let color_difference = self.end - self.start;

        self.start + (color_difference * distance_from_beginning)
    }
}

impl Eq for GradientPattern {}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn the_color_changes_linearly_along_the_x_axis() {
        let pattern = GradientPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(-0.5, 0.0, 0.0)), WHITE);
        assert_eq!(
            pattern.color_at(&Point::new(-0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.0, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.25, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn the_color_does_not_change_on_the_y_or_z_axes() {
        let pattern = GradientPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(-0.5, 0.25, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(-0.5, 0.25, 0.25)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(-0.5, 0.0, 0.25)), WHITE);
    }

    #[test]
    fn the_gradient_can_be_stretched_by_scaling_it() {
        let mut pattern = GradientPattern::new(WHITE, BLACK);
        pattern.set_transform(Transform::scaling(2.0, 1.0, 1.0));

        assert_eq!(pattern.color_at(&Point::new(-1.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            pattern.color_at(&Point::new(-0.5, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.0, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.5, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn the_gradient_can_be_made_vertical_by_rotating_it() {
        let mut pattern = GradientPattern::new(WHITE, BLACK);
        pattern.set_transform(Transform::y_rotation(-PI / 2.0));

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, -0.5)), WHITE);
        assert_eq!(
            pattern.color_at(&Point::new(0.0, 0.0, -0.25)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.0, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.0, 0.0, 0.25)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
