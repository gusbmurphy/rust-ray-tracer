pub use crate::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct BlendedPattern<const S: usize> {
    patterns: [Rc<dyn Pattern>; S],
}

impl<const S: usize> BlendedPattern<S> {
    pub fn new(patterns: [Rc<dyn Pattern>; S]) -> Self {
        BlendedPattern { patterns }
    }
}

impl<const S: usize> Pattern for BlendedPattern<S> {
    fn color_at(&self, point: &Point) -> Color {
        let mut resulting_color = self.patterns[0].color_at(point);

        for i in 1..S {
            resulting_color = resulting_color * self.patterns[i].color_at(point);
        }

        resulting_color
    }
}

impl<const S: usize> Transformable for BlendedPattern<S> {
    fn set_transform(&mut self, _transform: Transform) {
        todo!()
    }
}

impl<const S: usize> Eq for BlendedPattern<S> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blending_two_solid_patterns_returns_the_combination_of_the_colors() {
        let pattern_1 = Rc::new(FlatPattern::new(Color::new(1.0, 0.2, 0.4)));
        let pattern_2 = Rc::new(FlatPattern::new(Color::new(0.9, 1.0, 0.1)));

        let pattern = BlendedPattern::new([pattern_1, pattern_2]);

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

        let pattern = BlendedPattern::new([pattern_1, pattern_2]);

        assert_eq!(pattern.color_at(&ORIGIN), Color::new(0.9, 0.2, 0.04))
    }
}
