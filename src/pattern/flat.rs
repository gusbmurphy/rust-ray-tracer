use crate::prelude::*;

use super::pattern::Pattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatPattern {
    color: Color,
}

impl FlatPattern {
    pub fn new(color: Color) -> Self {
        FlatPattern { color }
    }
}

impl Pattern for FlatPattern {
    fn color_at(&self, _point: &Point) -> Color {
        self.color.to_owned()
    }
}

impl Transformable for FlatPattern {
    fn set_transform(&mut self, _transform: Transform) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_pattern_is_constant_everywhere() {
        let pattern = FlatPattern::new(WHITE);

        assert_eq!(pattern.color_at(&Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(1.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(&Point::new(0.0, 0.0, 1.0)), WHITE);
    }
}
