use crate::prelude::*;

use super::pattern::Pattern;

struct StripePattern {
    width: f32,
    background: Color,
    stripe: Color,
}

impl StripePattern {
    fn new(background: Color, stripe: Color, width: f32) -> Self {
        StripePattern {
            width,
            background,
            stripe,
        }
    }
}

impl Pattern for StripePattern {
    fn color_at(&self, point: &Point) -> Color {
        let point_x = point.x().to_owned();

        if (point_x % (self.width * 2.0)).floor() == 0.0 {
            self.background.clone()
        } else {
            self.stripe.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_pattern_is_constant_on_the_y_axis() {
        let pattern = StripePattern::new(WHITE, BLACK, 1.0);

        assert_eq!(pattern.color_at(&Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 2.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 3.0, 0.0)), WHITE);
    }

    #[test]
    fn the_pattern_is_constant_on_the_z_axis() {
        let pattern = StripePattern::new(WHITE, BLACK, 1.0);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 2.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 3.0)), WHITE);
    }

    #[test]
    fn the_color_changes_along_the_x_axis() {
        let pattern = StripePattern::new(WHITE, BLACK, 1.0);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    }
}
