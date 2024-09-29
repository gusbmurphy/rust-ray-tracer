use crate::prelude::*;

use super::refractive::determine_refractive_indexes;

pub fn schlick_approximation(intersection: &Intersection) -> f64 {
    let eye_vector = -intersection.ray().direction().to_owned();
    let normal = intersection.normal_vector();
    let cos = dot(&eye_vector, &normal);

    let all_intersections = Intersection::of(intersection.object(), intersection.ray());

    let refractive_indexes = determine_refractive_indexes(intersection.t(), all_intersections);

    if refractive_indexes[0] > refractive_indexes[1] {
        let n = refractive_indexes[0] / refractive_indexes[1];
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
        if sin2_t > 1.0 {
            return 1.0;
        }
    }

    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn reflectance_is_1_under_total_internal_reflection() {
        let sphere = Rc::new(Sphere::new_with_material(
            MaterialBuilder::new()
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        )) as Rc<dyn Shape>;

        let ray = Ray::new(Point::new(0.0, 0.0, 2f64.sqrt() / 2f64), POSITIVE_Y);

        let intersections = Intersection::of(&sphere, &ray);
        let target_intersection = intersections.get(1).unwrap();

        assert_eq!(schlick_approximation(target_intersection), 1.0);
    }
}
