use crate::prelude::*;

use super::pattern::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct TestPattern {
    transform: Transform,
}

impl TestPattern {
    pub fn new() -> Self {
        TestPattern {
            transform: Transform::new(IDENTITY_MATRIX),
        }
    }
}

impl Pattern for TestPattern {
    fn color_at(&self, point: &Point) -> Color {
        Color::new(*point.x(), *point.y(), *point.z())
    }
}

impl Eq for TestPattern {}

impl Transformable for TestPattern {
    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_returned_color_is_the_same_as_the_coordinates_of_the_hit() {
        let pattern = TestPattern::new();

        assert_eq!(
            pattern.color_at(&Point::new(0.0, 1.0, 0.0)),
            Color::new(0.0, 1.0, 0.0)
        );
        assert_eq!(
            pattern.color_at(&Point::new(1.0, 0.0, 0.0)),
            Color::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            pattern.color_at(&Point::new(0.0, 0.0, 1.0)),
            Color::new(0.0, 0.0, 1.0)
        );
    }
}
