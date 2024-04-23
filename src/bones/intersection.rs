use std::borrow::Borrow;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Intersection<'a, T> {
    time: f32,
    object: &'a T,
}

impl<'a, T> Intersection<'a, T>
where
    T: Intersectable,
{
    pub fn new(time: f32, object: &'a T) -> Self {
        Intersection { time, object }
    }

    pub fn get_intersected(self) -> &'a T {
        self.object
    }

    pub fn get_t(&self) -> f32 {
        self.time
    }
}

pub trait Intersectable {}

pub fn determine_hit<'a, const S: usize, T>(
    intersections: [&'a Intersection<'a, T>; S],
) -> Option<&'a Intersection<'a, T>>
where
    T: Intersectable,
{
    let mut lowest_t_intersection: &Intersection<T> = intersections.first().unwrap();

    for intersection in intersections {
        if intersection.get_t() < lowest_t_intersection.get_t() {
            lowest_t_intersection = intersection.borrow();
        }
    }

    return Some(lowest_t_intersection);
}

#[cfg(test)]
mod test {
    use crate::bones::sphere::Sphere;

    use super::*;

    #[test]
    fn the_lowest_positive_t_among_positives_is_the_hit() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(1.0, &interesected_sphere);
        let i2 = Intersection::new(2.0, &interesected_sphere);

        let result = determine_hit([&i1, &i2]).unwrap();

        assert_eq!(result.to_owned(), i1);
    }
}
