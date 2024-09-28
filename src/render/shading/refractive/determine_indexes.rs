use crate::prelude::*;
use std::rc::Rc;

/// Determines the refractive indexes of the materials exited and entered (in that order) for a given `Intersection`. If there is no material exited, or one not entered, a value of `1.0` will be given.
///
/// The intersection is specified with the provided `t` value.
///
/// In the returned array, the first value is the refractive index of the exited object, and the second is of the entered one.
pub fn determine_refractive_indexes(
    target_t: &f64,
    mut all_intersections: Vec<Intersection>,
) -> [f64; 2] {
    all_intersections.sort_by(|a, b| a.t().partial_cmp(b.t()).unwrap());

    let mut exited_ri = 1.0;
    let mut entered_ri = 1.0;

    let hit = all_intersections
        .iter()
        .find(|intersection| intersection.t() == target_t);
    let mut shapes_entered: Vec<Rc<dyn Shape>> = Vec::new();

    for (_index, intersection) in all_intersections.iter().enumerate() {
        if hit.as_ref().is_some_and(|hit| *hit == intersection) {
            if let Some(last_shape_entered) = shapes_entered.last() {
                exited_ri = *last_shape_entered.material().refractive_index();
            } else {
                exited_ri = 1.0;
            }
        }

        let this_shape = intersection.object();
        if let Some(record_of_shape_being_entered) = shapes_entered
            .iter()
            .enumerate()
            .find(|&shape| shape.1 == this_shape)
        {
            shapes_entered.remove(record_of_shape_being_entered.0);
        } else {
            shapes_entered.push(this_shape.clone());
        }

        if hit.as_ref().is_some_and(|hit| *hit == intersection) {
            if let Some(last_shape_entered) = shapes_entered.last() {
                entered_ri = *last_shape_entered.material().refractive_index();
            } else {
                entered_ri = 1.0;
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
    fn indexes_entering_just_one_sphere() {
        let sphere: Rc<dyn Shape> = Rc::new(Sphere::new_with_material(
            MaterialBuilder::new().refractive_index(3.0).build(),
        ));
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersection::of(&sphere, &ray);
        let t = intersections.get(0).unwrap().t();

        let result = determine_refractive_indexes(t, intersections.clone());
        assert_eq!(result[0], 1.0); // 1.0 since there is no material being exited.
        assert_eq!(result[1], 3.0);
    }

    #[test]
    fn exiting_a_sphere_and_entering_an_internal_one() {
        let mut outer =
            Sphere::new_with_material(MaterialBuilder::new().refractive_index(3.0).build());
        outer.set_transform(Transform::scaling(3.0, 3.0, 3.0));

        let inner = Sphere::new_with_material(MaterialBuilder::new().refractive_index(9.7).build());

        let ray = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0));
        let outer_rc = Rc::new(outer) as Rc<dyn Shape>;
        let inner_rc = Rc::new(inner) as Rc<dyn Shape>;

        let intersections_with_outer = Intersection::of(&outer_rc, &ray);
        let intersections_with_inner = Intersection::of(&inner_rc, &ray);
        let mut all_intersections = intersections_with_outer.clone();
        all_intersections.append(&mut intersections_with_inner.clone());

        let result = determine_refractive_indexes(
            intersections_with_inner.get(0).unwrap().t(),
            all_intersections,
        );
        assert_eq!(
            result[0], 3.0,
            "the exited index should be the outer sphere's"
        );
        assert_eq!(
            result[1], 9.7,
            "the entered index should be the inner sphere's"
        );
    }

    #[test]
    fn complex_intersection_with_three_spheres() {
        // This is a scene with two spheres inside of one larger one, the two spheres also overlap.
        let mut a = Sphere::new_with_material(MaterialBuilder::new().refractive_index(3.0).build());
        a.set_transform(Transform::scaling(3.0, 3.0, 3.0));

        let mut b = Sphere::new_with_material(MaterialBuilder::new().refractive_index(2.0).build());
        b.set_transform(Transform::translation(0.0, 0.0, -0.5));

        let mut c = Sphere::new_with_material(MaterialBuilder::new().refractive_index(5.0).build());
        c.set_transform(Transform::translation(0.0, 0.0, 0.5));

        let ray = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0));
        let a_rc = Rc::new(a) as Rc<dyn Shape>;
        let b_rc = Rc::new(b) as Rc<dyn Shape>;
        let c_rc = Rc::new(c) as Rc<dyn Shape>;

        let mut intersections = Intersection::of(&a_rc, &ray);
        let b_intersections = Intersection::of(&b_rc, &ray);
        intersections.append(&mut b_intersections.clone());
        intersections.append(&mut Intersection::of(&c_rc, &ray));

        let t = b_intersections.get(1).unwrap().t();

        let result = determine_refractive_indexes(t, intersections);
        // For this intersection, the "C" sphere will be on both sides of the intersection, so it's
        // refractive index will be both values.
        assert_eq!(result[0], 5.0, "the first index should be \"C\"s");
        assert_eq!(result[1], 5.0, "and the second index should be \"C\"s");
    }
}
