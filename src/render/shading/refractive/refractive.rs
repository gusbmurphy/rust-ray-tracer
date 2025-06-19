use crate::{
    prelude::*,
    render::shading::shade_ray::{adjust_hit_under, shade_ray_with_maximum_recursion},
};

use super::determine_indexes::determine_refractive_indexes;

pub fn calculate_refractive_contribution(
    hit: &Intersection,
    world: &World,
    current_recursion_count: i8,
) -> Color {
    if *hit.material().transparency() == 0.0f64 {
        return BLACK;
    }

    let ray = hit.ray();
    let all_intersections = world.intersections_for(ray);
    let refractive_indexes = determine_refractive_indexes(hit.t(), all_intersections);

    let refractive_ratio = refractive_indexes[0] / refractive_indexes[1];

    let eye_vector = -hit.ray().direction().to_owned();
    let normal_vector = hit.normal_vector();
    let cos_i = dot(&eye_vector, &normal_vector);

    let sin2_t = refractive_ratio.powi(2) * (1.0 - cos_i.powi(2));
    if sin2_t > 1.0f64 {
        return BLACK;
    }

    let cos_t = (1.0 - sin2_t).sqrt();

    let direction_of_refracted_ray =
        normal_vector * (refractive_ratio * cos_i - cos_t) - eye_vector * refractive_ratio;

    let refracted_ray = Ray::new(adjust_hit_under(hit), direction_of_refracted_ray);

    shade_ray_with_maximum_recursion(world, &refracted_ray, current_recursion_count + 1)
        * *hit.material().transparency()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn an_opaque_object_has_no_refractive_contribution() {
        let world = World::create_default();
        let object = world.shapes().get(0).unwrap();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersection::of(object, &ray);

        let result = calculate_refractive_contribution(intersections.get(0).unwrap(), &world, 0);

        assert_eq!(result, BLACK);
    }

    #[test]
    fn black_is_returned_at_total_internal_reflection() {
        let mut world = World::create_default();
        let sphere = Sphere::new_with_material(
            MaterialBuilder::new()
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        );
        let sphere_rc = Rc::new(sphere) as Rc<dyn Shape>;
        world.set_shapes(vec![sphere_rc.clone()]);

        let ray = Ray::new(Point::new(0.0, 0.0, 2f64.sqrt() / 2f64), POSITIVE_Y);

        let intersections = Intersection::of(&sphere_rc, &ray);
        let hit = intersections.get(1).unwrap();

        let result = calculate_refractive_contribution(hit, &world, 0);
        assert_eq!(result, BLACK);
    }
}
