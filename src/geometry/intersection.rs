use crate::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Intersection<'o, 'r, O> {
    time: f32,
    object: &'o O,
    ray: &'r Ray,
}

impl<'o, 'r, O> Intersection<'o, 'r, O>
where
    O: Intersectable,
{
    pub fn new(time: f32, object: &'o O, ray: &'r Ray) -> Self {
        Intersection { time, object, ray }
    }

    pub fn intersected_object(&self) -> &'o O {
        self.object
    }

    pub fn t(&self) -> &f32 {
        &self.time
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn object(&self) -> &O {
        &self.object
    }

    pub fn point(&self) -> Point {
        self.ray.position_at(self.time)
    }

    pub fn normal_vector(&self) -> Vector {
        self.object.normal_at(self.point())
    }

    pub fn is_inside_object(&self) -> bool {
        dot(&-self.ray.direction().to_owned(), &self.normal_vector()) < 0f32
    }
}

pub trait Intersectable {
    fn normal_at(&self, world_space_point: Point) -> Vector;
}

pub fn determine_hit<'o, 'r, O>(
    intersections: Vec<Intersection<'o, 'r, O>>,
) -> Option<Intersection<'o, 'r, O>>
where
    O: Intersectable,
{
    let mut lowest_t_intersection: Option<Intersection<O>> = None;

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
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(1.0, &interesected_sphere, &ray);
        let i2 = Intersection::new(2.0, &interesected_sphere, &ray);

        let result = determine_hit(vec![i1, i2]).unwrap();

        assert_eq!(result.to_owned(), i1);
    }

    #[test]
    fn the_lowest_positive_t_is_the_hit_when_a_negative_is_present() {
        let interesected_sphere = Sphere::new();
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-1.0, &interesected_sphere, &ray);
        let i2 = Intersection::new(2.0, &interesected_sphere, &ray);
        let i3 = Intersection::new(10.0, &interesected_sphere, &ray);

        let result = determine_hit(vec![i1, i2, i3]).unwrap();

        assert_eq!(result.to_owned(), i2);
    }

    #[test]
    fn there_is_no_hit_if_every_t_is_negative() {
        let interesected_sphere = Sphere::new();
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-1.0, &interesected_sphere, &ray);
        let i2 = Intersection::new(-2.0, &interesected_sphere, &ray);

        let result = determine_hit(vec![i1, i2]);

        assert!(result.is_none())
    }

    #[test]
    fn let_me_say_it_again_there_is_no_hit_if_every_t_is_negative() {
        let interesected_sphere = Sphere::new();
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-1.07378995, &interesected_sphere, &ray);
        let i2 = Intersection::new(-2.38418579E-7, &interesected_sphere, &ray);

        let result = determine_hit(vec![i1, i2]);

        assert!(result.is_none())
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let interesected_sphere = Sphere::new();
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(5.0, &interesected_sphere, &ray);
        let i2 = Intersection::new(7.0, &interesected_sphere, &ray);
        let i3 = Intersection::new(-3.0, &interesected_sphere, &ray);
        let i4 = Intersection::new(2.0, &interesected_sphere, &ray);

        let result = determine_hit(vec![i1, i2, i3, i4]).unwrap();

        assert_eq!(result.to_owned(), i4)
    }

    #[test]
    fn the_intersection_says_that_we_are_outside_of_the_shape() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, &sphere, &ray);

        assert_eq!(intersection.is_inside_object(), false);
    }

    #[test]
    fn the_intersection_says_we_are_inside_the_shape() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(1.0, &sphere, &ray);

        assert_eq!(intersection.is_inside_object(), true);
    }
}
