use crate::prelude::*;

pub struct Sphere {
    center: Point,
    radius: f32
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0
        }
    }
}
