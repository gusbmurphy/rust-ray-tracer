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
    let mut lowest_t_intersection: Option<&Intersection<T>> = None;

    for intersection in intersections {
        if intersection.get_t() > -1f32 {
            match lowest_t_intersection {
                None => lowest_t_intersection = Some(intersection),
                Some(lowest_t) => {
                    if intersection.get_t() < lowest_t.get_t() {
                        lowest_t_intersection = Some(intersection.borrow());
                    }
                }
            }
        }
    }

    return lowest_t_intersection;
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

    #[test]
    fn the_lowest_positive_t_is_the_hit_when_a_negative_is_present() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(-1.0, &interesected_sphere);
        let i2 = Intersection::new(2.0, &interesected_sphere);

        let result = determine_hit([&i1, &i2]).unwrap();

        assert_eq!(result.to_owned(), i2);
    }

    #[test]
    fn there_is_no_hit_if_every_t_is_negative() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(-1.0, &interesected_sphere);
        let i2 = Intersection::new(-2.0, &interesected_sphere);

        let result = determine_hit([&i1, &i2]);

        assert!(result.is_none())
    }
}
