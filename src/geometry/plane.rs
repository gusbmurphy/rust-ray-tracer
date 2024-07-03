use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    transform: Transform,
    material: Material,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: Transform::new(IDENTITY_MATRIX),
            material: Material::new(),
        }
    }
}

impl Shape for Plane {
    fn normal_at(&self, _world_space_point: Point) -> Vector {
        self.transform * POSITIVE_Y
    }

    fn intersections_with<'s, 'r>(&'s self, ray: &'r Ray) -> Vec<Intersection>
    where
        'r: 's,
    {
        let local_ray = self.transform.invert().unwrap() * ray;

        if local_ray.direction().y().abs() < EPSILON {
            return vec![];
        }

        let t = -local_ray.origin().y() / local_ray.direction().y();
        let intersection = Intersection::new(t, Box::new(*self), &ray);

        vec![intersection]
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transformation: Transform) {
        self.transform = transformation;
    }

    fn shape_type(&self) -> ShapeType {
        ShapeType::Plane
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn the_normal_vector_is_always_the_same() {
        let plane = Plane::new();

        assert_eq!(plane.normal_at(ORIGIN), POSITIVE_Y);
        assert_eq!(plane.normal_at(Point::new(1.0, 0.0, 0.0)), POSITIVE_Y);
        assert_eq!(plane.normal_at(Point::new(8.0, 0.0, -3.0)), POSITIVE_Y);
    }

    #[test]
    fn there_are_no_intersections_with_a_parallel_ray() {
        // Given a flat plane, and a ray above it running parallel...
        let plane = Plane::new();
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), POSITIVE_Z);

        let intersections = plane.intersections_with(&ray);

        assert!(intersections.is_empty())
    }

    #[test]
    fn there_are_no_intersections_with_a_coplanar_ray() {
        // Given a flat plane, and a ray at the same height running parallel...
        let plane = Plane::new();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), POSITIVE_Z);

        let intersections = plane.intersections_with(&ray);

        assert!(intersections.is_empty())
    }

    #[test]
    fn a_ray_intersection_from_above() {
        let plane = Plane::new();
        let ray = Ray::new(Point::new(0.0, 3.0, 0.0), NEGATIVE_Y);

        let intersections = plane.intersections_with(&ray);

        assert_eq!(intersections.len(), 1);

        let intersection = intersections.get(0).unwrap();
        assert_eq!(*intersection.t(), 3.0);
    }

    #[test]
    fn a_ray_intersection_from_below() {
        let plane = Plane::new();
        let ray = Ray::new(Point::new(0.0, -3.0, 0.0), POSITIVE_Y);

        let intersections = plane.intersections_with(&ray);

        assert_eq!(intersections.len(), 1);

        let intersection = intersections.get(0).unwrap();
        assert_eq!(*intersection.t(), 3.0);
    }

    #[test]
    fn intersection_when_plane_is_up_and_down() {
        let mut plane = Plane::new();
        plane.set_transform(
            Transform::new_translation(0.0, 0.0, 2.0) * Transform::new_x_rotation(PI / 2.0),
        );

        let ray = Ray::new(ORIGIN, POSITIVE_Z);

        let intersections = plane.intersections_with(&ray);

        assert_eq!(intersections.len(), 1);

        let intersection = intersections.get(0).unwrap();
        assert_eq!(*intersection.t(), 2.0);
    }
}
