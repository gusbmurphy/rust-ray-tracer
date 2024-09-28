use crate::prelude::*;
use std::rc::Rc;

/// Determines the refractive indexes of the materials exited and entered (in that order) for a given `Intersection`. If there is no material exited, or one not entered, a value of `1.0` will be given.
///
/// The intersection is specified with the provided `t` value.
///
/// In the returned array, the first value is the refractive index of the exited object, and the second is of the entered one.
pub fn determine_refractive_indexes(
    t: &f64,
    ray: &Ray,
    all_intersections: &Vec<Intersection>,
) -> [f64; 2] {
    let mut exited_ri = 1.0;
    let mut entered_ri = 1.0;

    let hit = determine_hit(all_intersections.to_owned().to_vec());
    let mut shapes_entered: Vec<Rc<dyn Shape>> = Vec::new();

    for (index, intersection) in all_intersections.iter().enumerate() {
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
    fn indexes_entering_just_one_sphere() {
        let sphere: Rc<dyn Shape> = Rc::new(Sphere::new_with_material(
            MaterialBuilder::new().refractive_index(3.0).build(),
        ));
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersection::of(&sphere, &ray);
        let t = intersections.get(0).unwrap().t();

        let result = determine_refractive_indexes(t, &ray, &intersections);
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

        let mut intersections = Intersection::of(&outer_rc, &ray);
        intersections.append(&mut Intersection::of(&inner_rc, &ray));

        let result = determine_refractive_indexes(&1.5, &ray, &intersections);
        assert_eq!(result[0], 3.0);
        assert_eq!(result[1], 9.7);
    }

    #[test]
    fn complex_intersection_with_three_spheres() {
        // This is a scene with two spheres inside of one larger one, the two spheres also overlap.
        let mut a = Sphere::new_with_material(MaterialBuilder::new().refractive_index(3.0).build());
        a.set_transform(Transform::scaling(3.0, 3.0, 3.0));

        let mut b = Sphere::new_with_material(MaterialBuilder::new().refractive_index(2.0).build());
        b.set_transform(Transform::translation(0.0, 0.0, -0.5));

        let mut c = Sphere::new_with_material(MaterialBuilder::new().refractive_index(1.0).build());
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

        let result = determine_refractive_indexes(t, &ray, &intersections);
        // For this intersection, the "C" sphere will be on both sides of the intersection, so it's
        // refractive index will be both values.
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 1.0);
    }
}
