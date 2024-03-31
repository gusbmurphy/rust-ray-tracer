use crate::bones::sphere::Sphere;
use crate::prelude::*;

use super::vector::dot;

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn get_position(&self, time: f32) -> Point {
        self.origin + self.direction * time
    }

    pub fn intersections_with(&self, sphere: Sphere) -> Option<[f32; 2]> {
        let vector_from_sphere_to_ray = self.origin - sphere.get_center();

        let a = dot(&self.direction, &self.direction);
        let b = 2f32 * dot(&self.direction, &vector_from_sphere_to_ray);
        let c = dot(&vector_from_sphere_to_ray, &vector_from_sphere_to_ray) - 1f32;

        let discriminant = b.powi(2) - 4f32 * a * c;

        if discriminant < 0f32 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2f32 * a);
        let t2 = (-b + discriminant.sqrt()) / (2f32 * a);

        return Some([t1, t2]);
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

        let intersections = ray.intersections_with(sphere);

        assert_eq!(intersections, Some([4.0, 6.0]));
    }
}
