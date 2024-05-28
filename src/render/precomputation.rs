use crate::prelude::*;

pub struct Precomputation<'i, 'r, 'o, O>
where
    O: Intersectable + 'o,
    'o: 'i,
{
    intersection: &'i Intersection<'o, O>,
    ray: &'r Ray,
}

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
}
