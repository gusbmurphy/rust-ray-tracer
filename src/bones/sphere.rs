use crate::prelude::*;

use super::{intersection::Intersectable, matrix::IDENTITY_MATRIX, transform::Transform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f32,
    transform: Transform,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Transform::new(IDENTITY_MATRIX),
        }
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_transform(&mut self, transformation: Transform) {
        self.transform = transformation;
    }
}

impl Intersectable for Sphere {}

#[cfg(test)]
mod test {
    use crate::bones::ray::Ray;

    use super::*;

    #[test]
    fn default_sphere_transform() {
        let sphere = Sphere::new();
        assert_eq!(sphere.get_transform().to_owned(), IDENTITY_MATRIX);
    }

    #[test]
    fn changing_sphere_transform() {
        let mut sphere = Sphere::new();
        let translation = Transform::new_translation(2.0, 2.0, 4.0);

        sphere.set_transform(translation);

        assert!(translation == sphere.get_transform().to_owned());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::new_scaling(2.0, 2.0, 2.0));

        let intersections = ray.intersections_with(&sphere).unwrap();

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].get_t(), 3.0);
        assert_eq!(intersections[1].get_t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::new_translation(5.0, 0.0, 0.0));

        let intersections = ray.intersections_with(&sphere);

        assert!(intersections.is_none());
    }
}
