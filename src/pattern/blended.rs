pub use crate::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct BlendedPattern {
    patterns: Vec<Rc<dyn Pattern>>,
}

impl BlendedPattern {
    pub fn new(patterns: Vec<Rc<dyn Pattern>>) -> Self {
        BlendedPattern { patterns }
    }
}

impl Pattern for BlendedPattern {
    fn color_at(&self, point: &Point) -> Color {
        let patterns_slice = self.patterns.as_slice();
        let mut resulting_color = patterns_slice[0].color_at(point);

        for i in 1..patterns_slice.len() {
            resulting_color = resulting_color * patterns_slice[i].color_at(point);
        }

        resulting_color
    }
}

impl Transformable for BlendedPattern {
    fn set_transform(&mut self, _transform: Transform) {
        todo!()
    }
}

impl Eq for BlendedPattern {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blending_two_solid_patterns_returns_the_combination_of_the_colors() {
        let pattern_1 = Rc::new(FlatPattern::new(Color::new(1.0, 0.2, 0.4)));
        let pattern_2 = Rc::new(FlatPattern::new(Color::new(0.9, 1.0, 0.1)));

        let pattern = BlendedPattern::new(vec![pattern_1, pattern_2]);

        assert_eq!(pattern.color_at(&ORIGIN), Color::new(0.9, 0.2, 0.04))
    }

    #[test]
    fn blending_two_checkered_patterns_returns_the_combinations_of_both_colors() {
        let pattern_1 = Rc::new(Checker3DPattern::new(
            Color::new(1.0, 0.2, 0.4),
            Color::new(0.9, 1.0, 0.1),
        ));
        let pattern_2 = Rc::new(Checker3DPattern::new(
            Color::new(0.9, 1.0, 0.1),
            Color::new(1.0, 0.2, 0.4),
        ));

        let pattern = BlendedPattern::new(vec![pattern_1, pattern_2]);

        assert_eq!(pattern.color_at(&ORIGIN), Color::new(0.9, 0.2, 0.04))
    }
}
