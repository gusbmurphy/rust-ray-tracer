use crate::prelude::*;

use super::pattern::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    background: Color,
    stripe: Color,
}

impl StripePattern {
    pub fn new(background: Color, stripe: Color) -> Self {
        StripePattern { background, stripe }
    }
}

impl Pattern for StripePattern {
    fn color_at(&self, point: &Point) -> Color {
        let point_x = point.x().to_owned();

        if (point_x.floor() % 2.0) == 0.0 {
            self.background.clone()
        } else {
            self.stripe.clone()
        }
    }
}

impl Eq for StripePattern {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_pattern_is_constant_on_the_y_axis() {
        let pattern = StripePattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 2.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 3.0, 0.0)), WHITE);
    }

    #[test]
    fn the_pattern_is_constant_on_the_z_axis() {
        let pattern = StripePattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 2.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 3.0)), WHITE);
    }

    #[test]
    fn the_color_changes_along_the_x_axis() {
        let pattern = StripePattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    }
}
