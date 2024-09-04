use crate::prelude::*;
use crate::render::shading::shade_ray::adjust_hit;

pub fn calculate_diffuse_contribution(light: &PointLight, hit: &Intersection) -> Color {
    let light_vector = (*light.position() - hit.point()).normalize();

    let light_dot_normal = dot(&light_vector, &hit.normal_vector());

    if light_dot_normal < 0.0 {
        return BLACK;
    }

    let adjusted_hit = adjust_hit(&hit);
    let hit_in_object_space = hit.object().transform().invert().unwrap() * adjusted_hit;

    let effective_color = hit.material().color_at(&hit_in_object_space) * *light.intensity();

    let light_dot_normal = dot(&light_vector, &hit.normal_vector());

    effective_color * *hit.material().diffuse() * light_dot_normal
}
