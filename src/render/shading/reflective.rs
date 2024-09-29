use crate::prelude::*;
use crate::render::shading::shade_ray::adjust_hit_over;
use crate::render::shading::shade_ray::shade_ray_with_maximum_recursion;

pub fn calculate_reflective_contribution(
    hit: &Intersection,
    world: &World,
    current_recursion_count: i8,
) -> Color {
    let adjusted_hit = adjust_hit_over(&hit);
    let material = hit.material();

    let reflection_vector = hit.ray().direction().reflect_around(&hit.normal_vector());

    shade_ray_with_maximum_recursion(
        world,
        &Ray::new(adjusted_hit, reflection_vector),
        current_recursion_count + 1,
    ) * *material.reflective()
}
