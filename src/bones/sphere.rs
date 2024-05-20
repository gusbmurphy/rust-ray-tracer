use crate::prelude::*;

use super::{
    intersection::Intersectable,
    matrix::{Matrix, IDENTITY_MATRIX},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f32,
    transform: Matrix<4>,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_transform(&self) -> &Matrix<4> {
        &self.transform
    }
}

impl Intersectable for Sphere {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_sphere_transformation() {
        let sphere = Sphere::new();
        assert_eq!(sphere.get_transform().to_owned(), IDENTITY_MATRIX);
    }
}
