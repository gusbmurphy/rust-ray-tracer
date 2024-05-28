use crate::prelude::*;

pub struct Precomputation<'i, 'r, 'o, O>
where
    O: Intersectable + 'o,
    'o: 'i,
{
    intersection: &'i Intersection<'o, O>,
    ray: &'r Ray,
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
        Precomputation {
            intersection,
            ray
        }
    }

    pub fn get_t(&self) -> f32 {
        self.intersection.get_t()
    }

    pub fn get_object(&self) -> &O {
        self.intersection.get_intersected()
    }

    pub fn get_hit_point(&self) -> Point {
        let t = self.intersection.get_t();
        return self.ray.get_position(t);
    }

    pub fn get_eye_vector(&self) -> Vector {
        -self.ray.get_direction().to_owned()
    }

    pub fn get_normal_vector(&self) -> Vector {
        self.intersection.get_intersected().normal_at(self.get_hit_point())
    }
}

#[cfg(test)]
mod test {
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
}
