use crate::prelude::*;

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

    pub fn intersected_object(&self) -> &'a T {
        self.object
    }

    pub fn t(&self) -> &f32 {
        &self.time
    }
}

pub trait Intersectable {
    fn normal_at(&self, world_space_point: Point) -> Vector;
}

pub fn determine_hit<'a, T>(intersections: Vec<Intersection<'a, T>>) -> Option<Intersection<'a, T>>
where
    T: Intersectable,
{
    let mut lowest_t_intersection: Option<Intersection<T>> = None;

    for intersection in intersections {
        if *intersection.t() > 0f32 {
            match lowest_t_intersection {
                None => lowest_t_intersection = Some(intersection),
                Some(ref lowest_t) => {
                    if intersection.t() < lowest_t.t() {
                        lowest_t_intersection = Some(intersection);
                    }
                }
            }
        }
    }

    return lowest_t_intersection;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::Sphere;

    #[test]
    fn the_lowest_positive_t_among_positives_is_the_hit() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(1.0, &interesected_sphere);
        let i2 = Intersection::new(2.0, &interesected_sphere);

        let result = determine_hit(vec![i1, i2]).unwrap();

        assert_eq!(result.to_owned(), i1);
    }

    #[test]
    fn the_lowest_positive_t_is_the_hit_when_a_negative_is_present() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(-1.0, &interesected_sphere);
        let i2 = Intersection::new(2.0, &interesected_sphere);
        let i3 = Intersection::new(10.0, &interesected_sphere);

        let result = determine_hit(vec![i1, i2, i3]).unwrap();

        assert_eq!(result.to_owned(), i2);
    }

    #[test]
    fn there_is_no_hit_if_every_t_is_negative() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(-1.0, &interesected_sphere);
        let i2 = Intersection::new(-2.0, &interesected_sphere);

        let result = determine_hit(vec![i1, i2]);

        assert!(result.is_none())
    }

    #[test]
    fn let_me_say_it_again_there_is_no_hit_if_every_t_is_negative() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(-1.07378995, &interesected_sphere);
        let i2 = Intersection::new(-2.38418579E-7, &interesected_sphere);

        let result = determine_hit(vec![i1, i2]);

        assert!(result.is_none())
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let interesected_sphere = Sphere::new();

        let i1 = Intersection::new(5.0, &interesected_sphere);
        let i2 = Intersection::new(7.0, &interesected_sphere);
        let i3 = Intersection::new(-3.0, &interesected_sphere);
        let i4 = Intersection::new(2.0, &interesected_sphere);

        let result = determine_hit(vec![i1, i2, i3, i4]).unwrap();

        assert_eq!(result.to_owned(), i4)
    }
}
