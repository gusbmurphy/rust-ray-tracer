use crate::prelude::*;

use super::intersection::Intersectable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn get_center(&self) -> Point {
        self.center
    }
}

impl Intersectable for Sphere {}
