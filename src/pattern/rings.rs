use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RingPattern {
    background: Color,
    stripe: Color,
    transform: Transform,
}

impl RingPattern {
    pub fn new(background: Color, stripe: Color) -> Self {
        let transform = Transform::new(IDENTITY_MATRIX);

        RingPattern {
            background,
            stripe,
            transform,
        }
    }
}

impl Pattern for RingPattern {
    fn color_at(&self, point: &Point) -> Color {
        let pattern_space_point = self.transform.invert().unwrap() * *point;
        let x = pattern_space_point.x();
        let z = pattern_space_point.z();

        // The hypotenuse of the triangle created by X and Z will be the distance from the center
        let distance_from_center = (x.powi(2) + z.powi(2)).sqrt();
        if distance_from_center.floor().rem_euclid(2.0) == 0.0 {
            self.background.clone()
        } else {
            self.stripe.clone()
        }
    }
}

impl Transformable for RingPattern {
    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl Eq for RingPattern {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_rings_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(WHITE, BLACK);

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), BLACK);
        // A point on that first ring...
        assert_eq!(pattern.color_at(&Point::new(0.708, 0.0, 0.708)), BLACK);
    }

    #[test]
    fn rings_can_be_scaled() {
        let mut pattern = RingPattern::new(WHITE, BLACK);
        pattern.set_transform(Transform::scaling(2.0, 2.0, 2.0));

        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(2.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 2.0)), BLACK);
        // A point on that first ring...
        assert_eq!(pattern.color_at(&Point::new(1.416, 0.0, 1.416)), BLACK);
    }
}
