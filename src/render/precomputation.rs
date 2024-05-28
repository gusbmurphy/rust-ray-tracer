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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_is_correctly_given_based_on_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -0.5), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.get_t(), 4.0);
    }

    #[test]
    fn the_object_is_correctly_given_based_on_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -0.5), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere);

        let computation = Precomputation::new(&intersection, &ray);

        assert_eq!(computation.get_object().to_owned(), sphere);
    }
}
