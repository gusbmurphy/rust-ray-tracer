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
}
