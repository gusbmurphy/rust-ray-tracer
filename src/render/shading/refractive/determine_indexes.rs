use crate::prelude::*;
use std::rc::Rc;

/// Determines the refractive indexes of the materials exited and entered (in that order) given a number of `Intersection`s. If there is no material exited, or one not entered, a value of `1.0` will be given.
pub fn determine_refractive_indexes(intersections: &Vec<Intersection>) -> [f64; 2] {
    let mut exited_ri = 1.0;
    let mut entered_ri = 1.0;

    let hit = determine_hit(intersections.to_owned().to_vec());
    let mut shapes_entered: Vec<Rc<dyn Shape>> = Vec::new();

    for (index, intersection) in intersections.iter().enumerate() {
        shapes_entered.push(intersection.object());

        if let Some(ref h) = hit {
            if h == intersection {
                entered_ri = *intersection.object().material().refractive_index();

                // A bit of a yikes here...
                if let Some(last_index) = index.checked_sub(1) {
                    if let Some(last_object_exited) = shapes_entered.get(last_index) {
                        exited_ri = *last_object_exited.material().refractive_index();
                    }
                }
            }
        }
    }

    [exited_ri, entered_ri]
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

        let result = determine_refractive_indexes(&intersections);
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

        let result = determine_refractive_indexes(&intersections);
        assert_eq!(result[0], 3.0);
        assert_eq!(result[1], 9.7);
    }
}
