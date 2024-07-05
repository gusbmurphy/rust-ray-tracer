use crate::prelude::*;

pub trait Pattern {
    fn color_at(&self, point: &Point) -> Color;
}
