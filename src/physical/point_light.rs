use crate::prelude::*;

pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    pub fn new(intensity: Color, position: Point) -> Self {
        PointLight { intensity, position }
    }

    pub fn get_intensity(&self) -> Color {
        self.intensity
    }

    pub fn get_position(&self) -> Point {
        self.position
    }
}