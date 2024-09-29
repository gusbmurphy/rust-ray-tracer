use crate::prelude::*;
use crate::render::shading::shade_ray::adjust_hit_over;

pub fn calculate_ambient_contribution(light: &PointLight, hit: &Intersection) -> Color {
    let adjusted_hit = adjust_hit_over(&hit);
    let hit_in_object_space = hit.object().transform().invert().unwrap() * adjusted_hit;

    let effective_color = hit.material().color_at(&hit_in_object_space) * *light.intensity();

    effective_color * hit.material().ambient()
}
