use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    pub fn new(intensity: Color, position: Point) -> Self {
        PointLight {
            intensity,
            position,
        }
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
    }

    pub fn position(&self) -> &Point {
        &self.position
    }
}
