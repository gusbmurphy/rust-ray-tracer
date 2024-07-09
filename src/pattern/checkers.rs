use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Checker3DPattern {
    background: Color,
    checker: Color,
    transform: Transform,
}

impl Checker3DPattern {
    pub fn new(background: Color, checker: Color) -> Self {
        Checker3DPattern {
            background,
            checker,
            transform: Transform::new(IDENTITY_MATRIX),
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl Pattern for Checker3DPattern {
    fn color_at(&self, point: &Point) -> Color {
        let pattern_space_point = self.transform.invert().unwrap() * *point;
        let x = pattern_space_point.x();
        let y = pattern_space_point.y();
        let z = pattern_space_point.z();

        let sum_of_coodinates_floors_is_even =
            (x.floor() + y.floor() + z.floor()).rem_euclid(2.0) == 0.0;

        if sum_of_coodinates_floors_is_even {
            self.background.clone()
        } else {
            self.checker.clone()
        }
    }
}

impl Eq for Checker3DPattern {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_pattern_repeats_in_the_x_direction() {
        let pattern = Checker3DPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn the_pattern_repeats_in_the_z_direction() {
        let pattern = Checker3DPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.9)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), BLACK);
    }

    #[test]
    fn the_pattern_repeats_in_the_y_direction() {
        let pattern = Checker3DPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.9, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 1.0, 0.0)), BLACK);
    }

    #[test]
    fn the_color_is_constant_diagonally() {
        let pattern = Checker3DPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.9, 0.0, 0.9)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.9, 0.0, 1.9)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(2.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn the_pattern_can_be_scaled() {
        let mut pattern = Checker3DPattern::new(WHITE, BLACK);
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
