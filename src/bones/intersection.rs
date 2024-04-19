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

pub fn determine_hit<const S: usize, T>(
    intersections: [Intersection<T>; S],
) -> Option<Intersection<T>>
where
    T: Intersectable,
{
    None
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

        let result = determine_hit([i1, i2]).unwrap();

        assert_eq!(result, i1);
    }
}
