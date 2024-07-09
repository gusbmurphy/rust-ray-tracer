use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CheckerPattern {
    background: Color,
    checker: Color,
    transform: Transform,
}

impl CheckerPattern {
    pub fn new(background: Color, checker: Color) -> Self {
        CheckerPattern {
            background,
            checker,
            transform: Transform::new(IDENTITY_MATRIX),
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl Pattern for CheckerPattern {
    fn color_at(&self, point: &Point) -> Color {
        let pattern_space_point = self.transform.invert().unwrap() * *point;
        let x = pattern_space_point.x();
        let z = pattern_space_point.z();

        let x_is_in_background = x.floor().rem_euclid(2.0) == 0.0;
        let z_is_in_background = z.floor().rem_euclid(2.0) == 0.0;

        if x_is_in_background && z_is_in_background {
            self.background.clone()
        } else {
            self.checker.clone()
        }
    }
}

impl Eq for CheckerPattern {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_pattern_repeats_in_the_x_and_z_directions() {
        let pattern = CheckerPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(1.9, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(2.0, 0.0, 0.0)), WHITE);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.9)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.9)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn the_pattern_is_constant_on_the_y_axis() {
        let pattern = CheckerPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 1.0, 0.0)), WHITE);
    }

    #[test]
    fn the_pattern_can_be_scaled() {
        let mut pattern = CheckerPattern::new(WHITE, BLACK);
        pattern.set_transform(Transform::scaling(0.5, 0.5, 0.5));

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.4, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.5, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(0.9, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), WHITE);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.4)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.5)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.9)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), WHITE);
    }
}
