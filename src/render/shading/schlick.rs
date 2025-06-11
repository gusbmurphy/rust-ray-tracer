use crate::prelude::*;

use super::refractive::determine_refractive_indexes;

pub fn schlick_approximation(intersection: &Intersection) -> f64 {
    let eye_vector = -intersection.ray().direction().to_owned();
    let normal = intersection.normal_vector();
    let mut cos = dot(&eye_vector, &normal);

    let all_intersections = Intersection::of(intersection.object(), intersection.ray());

    let refractive_indexes = determine_refractive_indexes(intersection.t(), all_intersections);

    if refractive_indexes[0] > refractive_indexes[1] {
        let n = refractive_indexes[0] / refractive_indexes[1];
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
        if sin2_t > 1.0 {
            return 1.0;
        }

        cos = (1.0 - sin2_t).sqrt();
    }

    let r0 = ((refractive_indexes[0] - refractive_indexes[1])
        / (refractive_indexes[0] + refractive_indexes[1]))
        .powi(2);

    return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn reflectance_is_1_under_total_internal_reflection() {
        let sphere = Arc::new(Sphere::new_with_material(
            MaterialBuilder::new()
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        )) as WorldShape;

        let ray = Ray::new(Point::new(0.0, 0.0, 2f64.sqrt() / 2f64), POSITIVE_Y);

        let intersections = Intersection::of(&sphere, &ray);
        let target_intersection = intersections.get(1).unwrap();

        assert_eq!(schlick_approximation(target_intersection), 1.0);
    }

    #[test]
    fn relfectance_of_perpindicular_ray() {
        let sphere = Arc::new(Sphere::new_with_material(
            MaterialBuilder::new()
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        )) as WorldShape;

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), POSITIVE_Y);

        let intersections = Intersection::of(&sphere, &ray);
        let target_intersection = intersections.get(1).unwrap();

        assert!(close_enough(
            &schlick_approximation(target_intersection),
            &0.04
        ));
    }

    #[test]
    fn relfectance_is_high_when_the_surface_is_struck_at_a_small_angle() {
        let sphere = Arc::new(Sphere::new_with_material(
            MaterialBuilder::new()
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        )) as WorldShape;

        let ray = Ray::new(Point::new(0.0, 0.99, -2.0), POSITIVE_Z);

        let intersections = Intersection::of(&sphere, &ray);
        let target_intersection = intersections.get(0).unwrap();

        assert!(close_enough(
            &schlick_approximation(target_intersection),
            &0.48881
        ));
    }
}
