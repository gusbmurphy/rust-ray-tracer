use crate::prelude::*;

pub struct Precomputation<'i, 'r, 'o, O>
where
    O: Intersectable + 'o,
    'o: 'i,
{
    intersection: &'i Intersection<'o, 'r, O>,
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
    pub fn new(intersection: &'i Intersection<'o, 'r, O>, ray: &'r Ray) -> Self {
        let hit_point = intersection.point();
        let base_normal_vector = intersection.normal_vector();

        Precomputation {
            intersection,
            ray,
            base_normal_vector,
            hit_point,
        }
    }

    pub fn t(&self) -> &f32 {
        self.intersection.t()
    }

    pub fn intersected_object(&self) -> &O {
        self.intersection.intersected_object()
    }

    pub fn hit_point(&self) -> &Point {
        &self.hit_point
    }

    pub fn eye_vector(&self) -> Vector {
        -self.intersection.ray().direction().to_owned()
    }

    pub fn normal_vector(&self) -> Vector {
        if self.is_inside() {
            -self.base_normal_vector
        } else {
            self.base_normal_vector
        }
    }

    pub fn is_inside(&self) -> bool {
        dot(&self.eye_vector(), &self.base_normal_vector) < 0f32
    }

    pub fn adjusted_hit_point(&self) -> Point {
        self.hit_point + self.normal_vector() * EPSILON
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_is_correctly_given_based_on_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(*computation.t(), 4.0);
    }

    #[test]
    fn the_object_is_correctly_given_based_on_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.intersected_object().to_owned(), sphere);
    }

    #[test]
    fn hit_point_is_correct() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(*computation.hit_point(), Point::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn eye_vector_is_the_opposite_of_the_given_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.eye_vector(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn normal_vector_is_based_on_the_hit_point_and_the_object() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.normal_vector(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_computation_correctly_says_that_we_are_outside_of_the_shape() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.is_inside(), false);
    }

    #[test]
    fn when_the_intersection_is_inside_the_vectors_are_correctly_calculated() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(1.0, &sphere, &ray);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.is_inside(), true);
        assert_eq!(*computation.hit_point(), Point::new(0.0, 0.0, 1.0));

        // And the normal is inverted...
        assert_eq!(computation.normal_vector(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_adjusted_hit_is_just_slightly_closer_to_the_origin_of_the_ray_than_the_actual_hit() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::new_translation(0.0, 0.0, 1.0));

        // The actual hit is right at the origin since the sphere is translated 1.0 in the positive
        // Z, and only has a radius of 1.0.
        let actual_hit = ORIGIN;

        let intersection = Intersection::new(5.0, &sphere, &ray);
        let computation = Precomputation::new(&intersection, &ray);
        let adjusted_hit = computation.adjusted_hit_point();

        // The actual hit and the adjusted one should have the same X and Y values...
        assert_eq!(adjusted_hit.x(), actual_hit.x());
        assert_eq!(adjusted_hit.y(), actual_hit.y());

        // ...but the Z value of the adjusted one should be less than the actual one because the
        // origin is in the negative Z direction...
        assert!(adjusted_hit.z() < actual_hit.z());
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
        let intersection = Intersection::new(3.0, &sphere, &ray);

        // ...then that hit (when adjusted) should not be shadowed.
        let computation = Precomputation::new(&intersection, &ray);
        let adjusted_hit = computation.adjusted_hit_point();

        assert_eq!(world.is_point_shadowed(&adjusted_hit), false);
    }
}
