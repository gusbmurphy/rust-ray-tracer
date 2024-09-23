use crate::prelude::*;

/// Determines the refractive indexes of the materials exited and entered (in that order) given a number of `Intersection`s. If there is no material exited, or one not entered, a value of `1.0` will be given.
pub fn determine_refractive_indexes(intersections: Vec<Intersection>) -> [f64; 2] {
    [
        1.0,
        *intersections.get(0).unwrap().material().refractive_index(),
    ]
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn indexes_with_just_one_sphere() {
        let sphere: Rc<dyn Shape> = Rc::new(Sphere::new_with_material(
            MaterialBuilder::new().refractive_index(3.0).build(),
        ));
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersection::of(&sphere, &ray);

        let result = determine_refractive_indexes(intersections);
        assert_eq!(result[0], 1.0); // 1.0 since there is no material being exited.
        assert_eq!(result[1], 3.0);
    }

    #[test]
    fn exiting_a_sphere_and_entering_an_internal_one() {
        let mut outer =
            Sphere::new_with_material(MaterialBuilder::new().refractive_index(3.0).build());
        outer.set_transform(Transform::scaling(3.0, 3.0, 3.0));

        let inner = Sphere::new_with_material(MaterialBuilder::new().refractive_index(9.7).build());

        let ray = Ray::new(Point::new(0.0, 0.0, -1.5), Vector::new(0.0, 0.0, 1.0));
        let outer_rc = Rc::new(outer) as Rc<dyn Shape>;
        let inner_rc = Rc::new(inner) as Rc<dyn Shape>;

        let mut intersections = Intersection::of(&outer_rc, &ray);
        intersections.append(&mut Intersection::of(&inner_rc, &ray));

        let result = determine_refractive_indexes(intersections);
        assert_eq!(result[0], 3.0);
        assert_eq!(result[1], 9.7);
    }
}
