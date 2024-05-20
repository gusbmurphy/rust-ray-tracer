use crate::prelude::*;

use super::{
    intersection::Intersectable,
    matrix::{Matrix, IDENTITY_MATRIX},
    transformation::Transformation
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

    pub fn set_transform(&mut self, transformation: Transformation) {
        self.transform = transformation.get_matrix().to_owned();
    }
}

impl Intersectable for Sphere {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_sphere_transform() {
        let sphere = Sphere::new();
        assert_eq!(sphere.get_transform().to_owned(), IDENTITY_MATRIX);
    }

    #[test]
    fn changing_sphere_transform() {
        let mut sphere = Sphere::new();
        let translation = Transformation::new_translation(2.0, 2.0, 4.0);

        sphere.set_transform(translation);

        assert!(translation == sphere.get_transform().to_owned() );
    }
}
