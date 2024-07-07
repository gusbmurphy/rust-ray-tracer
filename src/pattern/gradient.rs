use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GradientPattern {
    start: Color,
    end: Color,
}

impl GradientPattern {
    pub fn new(start: Color, end: Color) -> Self {
        GradientPattern { start, end }
    }
}

impl Pattern for GradientPattern {
    fn color_at(&self, point: &Point) -> Color {
        let color_difference = self.end - self.start;

        let point_x = point.x();
        let fractional_part_of_x = point_x - point_x.floor();

        self.start + (color_difference * fractional_part_of_x)
    }
}

impl Eq for GradientPattern {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_color_changes_linearly_along_the_x_axis() {
        let pattern = GradientPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            pattern.color_at(&Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn the_color_does_not_change_on_the_y_or_z_axes() {
        let pattern = GradientPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.25, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.25, 0.25)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.25)), WHITE);
    }
}
