use crate::prelude::*;

struct PointLight {
    color: Color,
    position: Point,
}

impl PointLight {
    pub fn new(color: Color, position: Point) -> Self {
        PointLight { color, position }
    }
}
