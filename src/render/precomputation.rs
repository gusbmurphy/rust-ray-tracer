use std::f32::EPSILON;

use crate::prelude::*;

pub struct Precomputation<'i, 'r, 'o, O>
where
    O: Intersectable + 'o,
    'o: 'i,
{
    intersection: &'i Intersection<'o, O>,
    ray: &'r Ray,
    base_normal_vector: Vector,
    hit_point: Point,
}

// TODO: It feels to me like maybe all of this can live just on
// the Intersection? Like why is it ever helpful to have an
// intersection with no Ray associated with it?
impl<'i, 'r, 'o, O> Precomputation<'i, 'r, 'o, O>
where
    O: Intersectable + 'o,
    'o: 'i,
{
    pub fn new(intersection: &'i Intersection<'o, O>, ray: &'r Ray) -> Self {
        let hit_point = Precomputation::calculate_hit_point(intersection, ray);
        let base_normal_vector = intersection.get_intersected().normal_at(hit_point);

        Precomputation {
            intersection,
            ray,
            base_normal_vector,
            hit_point,
        }
    }

    fn calculate_hit_point(intersection: &'i Intersection<'o, O>, ray: &'r Ray) -> Point {
        let t = intersection.get_t();
        return ray.get_position(t);
    }

    pub fn get_t(&self) -> f32 {
        self.intersection.get_t()
    }

    pub fn get_object(&self) -> &O {
        self.intersection.get_intersected()
    }

    pub fn get_hit_point(&self) -> Point {
        self.hit_point
    }

    pub fn get_eye_vector(&self) -> Vector {
        -self.ray.get_direction().to_owned()
    }

    pub fn get_normal_vector(&self) -> Vector {
        if self.is_inside() {
            -self.base_normal_vector
        } else {
            self.base_normal_vector
        }
    }

    pub fn is_inside(&self) -> bool {
        dot(&self.get_eye_vector(), &self.base_normal_vector) < 0f32
    }

    pub fn get_adjusted_hit_point(&self) -> Point {
        self.hit_point + self.get_normal_vector() * EPSILON
    }
}

#[cfg(test)]
mod test {
    use std::f32::EPSILON;

    use super::*;

    #[test]
    fn time_is_correctly_given_based_on_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.get_t(), 4.0);
    }

    #[test]
    fn the_object_is_correctly_given_based_on_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.get_object().to_owned(), sphere);
    }

    #[test]
    fn hit_point_is_correct() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.get_hit_point(), Point::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn eye_vector_is_the_opposite_of_the_given_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.get_eye_vector(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn normal_vector_is_based_on_the_hit_point_and_the_object() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.get_normal_vector(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_computation_correctly_says_that_we_are_outside_of_the_shape() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.is_inside(), false);
    }

    #[test]
    fn when_the_intersection_is_inside_the_vectors_are_correctly_calculated() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(1.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.is_inside(), true);
        assert_eq!(computation.get_hit_point(), Point::new(0.0, 0.0, 1.0));

        // And the normal is inverted...
        assert_eq!(computation.get_normal_vector(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_adjusted_hit_is_just_slightly_closer_to_the_origin_of_the_ray_than_the_actual_hit() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::new_translation(0.0, 0.0, 1.0));

        // The actual hit is right at the origin since the sphere is translated 1.0 in the positive
        // Z, and only has a radius of 1.0.
        let actual_hit = ORIGIN;

        let intersection = Intersection::new(5.0, &sphere);
        let computation = Precomputation::new(&intersection, &ray);
        let adjusted_hit = computation.get_adjusted_hit_point();

        // The actual hit and the adjusted one should have the same X and Y values...
        assert_eq!(adjusted_hit.get_x(), actual_hit.get_x());
        assert_eq!(adjusted_hit.get_y(), actual_hit.get_y());

        // ...but the Z value of the adjusted one should be less than the actual one because the
        // origin is in the negative Z direction...
        assert!(adjusted_hit.get_z() < actual_hit.get_z());
    }

    #[test]
    fn an_adjusted_hit_is_not_shadowed_by_the_object_it_hit() {
        // Given a world with a single sphere just off the origin and a light facing it...
        let mut world = World::new();

        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -5.0));
        world.set_light(light);

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::new_translation(0.0, 0.0, 1.0));

        // ...and a ray that hits the sphere at the origin...
        let ray = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0));
        let intersection = Intersection::new(3.0, &sphere);

        // ...then that hit (when adjusted) should not be shadowed.
        let computation = Precomputation::new(&intersection, &ray);
        let adjusted_hit = computation.get_adjusted_hit_point();

        assert_eq!(world.is_point_shadowed(&adjusted_hit), false);
    }
}
