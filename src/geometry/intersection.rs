use std::rc::Rc;

use crate::prelude::*;

pub struct Intersection<'r> {
    time: f64,
    object: Rc<dyn Shape>,
    ray: &'r Ray,
}

impl<'r> Intersection<'r> {
    pub fn new(time: f64, object: Rc<dyn Shape>, ray: &'r Ray) -> Self {
        Intersection { time, object, ray }
    }

    pub fn of(object: &Rc<dyn Shape>, ray: &'r Ray) -> Vec<Self> {
        let mut intersections = Vec::new();

        let intersection_times = object.times_of_intersections_with(&ray);

        for time in intersection_times {
            let intersection = Intersection::new(time, object.to_owned(), ray);
            intersections.push(intersection)
        }

        intersections
    }

    pub fn object(&self) -> Rc<dyn Shape> {
        self.object.clone()
    }

    pub fn material(&self) -> &Material {
        &self.object.material()
    }

    pub fn t(&self) -> &f64 {
        &self.time
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn point(&self) -> Point {
        self.ray.position_at(self.time)
    }

    pub fn normal_vector(&self) -> Vector {
        let base_normal = self.base_normal_vector();

        if self.is_inside_object() {
            -base_normal
        } else {
            base_normal
        }
    }

    fn base_normal_vector(&self) -> Vector {
        self.object.normal_at(self.point())
    }

    fn is_inside_object(&self) -> bool {
        dot(
            &-self.ray.direction().to_owned(),
            &self.base_normal_vector(),
        ) < 0f64
    }
}

pub fn determine_hit<'r>(intersections: Vec<Intersection>) -> Option<Intersection> {
    let mut lowest_t_intersection: Option<Intersection> = None;

    for intersection in intersections {
        if *intersection.t() > 0f64 {
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
        let interesected_sphere = Rc::new(Sphere::new());
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(1.0, interesected_sphere.clone(), &ray);
        let i2 = Intersection::new(2.0, interesected_sphere.clone(), &ray);

        let result = determine_hit(vec![i1, i2]).unwrap();

        assert_eq!(result.t().to_owned(), 1.0);
    }

    #[test]
    fn the_lowest_positive_t_is_the_hit_when_a_negative_is_present() {
        let interesected_sphere = Rc::new(Sphere::new());
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-1.0, interesected_sphere.clone(), &ray);
        let i2 = Intersection::new(2.0, interesected_sphere.clone(), &ray);
        let i3 = Intersection::new(10.0, interesected_sphere.clone(), &ray);

        let result = determine_hit(vec![i1, i2, i3]).unwrap();

        assert_eq!(result.t().to_owned(), 2.0);
    }

    #[test]
    fn there_is_no_hit_if_every_t_is_negative() {
        let interesected_sphere = Rc::new(Sphere::new());
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-1.0, interesected_sphere.clone(), &ray);
        let i2 = Intersection::new(-2.0, interesected_sphere.clone(), &ray);

        let result = determine_hit(vec![i1, i2]);

        assert!(result.is_none())
    }

    #[test]
    fn let_me_say_it_again_there_is_no_hit_if_every_t_is_negative() {
        let interesected_sphere = Rc::new(Sphere::new());
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-1.07378995, interesected_sphere.clone(), &ray);
        let i2 = Intersection::new(-2.38418579E-7, interesected_sphere.clone(), &ray);

        let result = determine_hit(vec![i1, i2]);

        assert!(result.is_none())
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let interesected_sphere = Rc::new(Sphere::new());
        let ray = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(5.0, interesected_sphere.clone(), &ray);
        let i2 = Intersection::new(7.0, interesected_sphere.clone(), &ray);
        let i3 = Intersection::new(-3.0, interesected_sphere.clone(), &ray);
        let i4 = Intersection::new(2.0, interesected_sphere.clone(), &ray);

        let result = determine_hit(vec![i1, i2, i3, i4]).unwrap();

        assert_eq!(result.t().to_owned(), 2.0)
    }

    #[test]
    fn the_normal_vector_when_the_hit_originates_outside_of_the_shape_is_correct() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4.0, Rc::new(sphere), &ray);

        assert_eq!(intersection.normal_vector(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_normal_vector_is_correct_when_we_are_inside_of_the_shape() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersection = Intersection::new(1.0, Rc::new(sphere), &ray);

        assert_eq!(intersection.normal_vector(), Vector::new(0.0, 0.0, -1.0));
    }
}
