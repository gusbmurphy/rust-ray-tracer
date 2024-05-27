use crate::prelude::*;

use super::intersection::Intersection;
use super::vector::dot;

#[derive(PartialEq, Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn get_origin(&self) -> &Point {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vector {
        &self.direction
    }

    pub fn get_position(&self, time: f32) -> Point {
        self.origin + self.direction * time
    }

    pub fn intersections_with<'a, 'b>(
        &'a self,
        sphere: &'b Sphere,
    ) -> Option<[Intersection<Sphere>; 2]>
    where
        'b: 'a,
    {
        let adjusted_ray = sphere.get_transform().invert().unwrap() * self;
        let vector_from_sphere_to_ray = *adjusted_ray.get_origin() - sphere.get_center();

        let a = dot(adjusted_ray.get_direction(), adjusted_ray.get_direction());
        let b = 2f32 * dot(adjusted_ray.get_direction(), &vector_from_sphere_to_ray);
        let c = dot(&vector_from_sphere_to_ray, &vector_from_sphere_to_ray) - 1f32;

        let discriminant = b.powi(2) - 4f32 * a * c;

        if discriminant < 0f32 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2f32 * a);
        let t2 = (-b + discriminant.sqrt()) / (2f32 * a);

        return Some([Intersection::new(t1, sphere), Intersection::new(t2, sphere)]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn getting_positions_on_ray() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(ray.get_position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.get_position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.get_position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.get_position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn intersection_through_middle_of_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let intersections = ray.intersections_with(&sphere);

        for intersection in intersections.unwrap() {
            assert_eq!(*intersection.get_intersected(), sphere);
        }

        intersections
            .unwrap()
            .iter()
            .any(|intersection| intersection.get_t() == 4.0);
        intersections
            .unwrap()
            .iter()
            .any(|intersection| intersection.get_t() == 6.0);
    }

    #[test]
    fn ray_missing_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let intersections = ray.intersections_with(&sphere);

        assert_eq!(intersections, None);
    }

    #[test]
    fn ray_originating_inside_of_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let intersections = ray.intersections_with(&sphere);

        for intersection in intersections.unwrap() {
            assert_eq!(*intersection.get_intersected(), sphere);
        }

        intersections
            .unwrap()
            .iter()
            .any(|intersection| intersection.get_t() == -1.0);
        intersections
            .unwrap()
            .iter()
            .any(|intersection| intersection.get_t() == 1.0);
    }

    #[test]
    fn ray_ahead_of_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let intersections = ray.intersections_with(&sphere);

        for intersection in intersections.as_ref().unwrap() {
            assert_eq!(*intersection.get_intersected(), sphere);
        }

        intersections
            .unwrap()
            .iter()
            .any(|intersection| intersection.get_t() == -6.0);
        intersections
            .unwrap()
            .iter()
            .any(|intersection| intersection.get_t() == -4.0);
    }
}
