use crate::prelude::*;

pub fn calculate_specular_contribution(light: &PointLight, hit: &Intersection) -> Color {
    let normal_vector = &hit.normal_vector();
    let material = hit.material();
    let light_vector = (*light.position() - hit.point()).normalize();

    let eye_vector = -hit.ray().direction().to_owned();

    let light_dot_normal = dot(&light_vector, &hit.normal_vector());

    if light_dot_normal < 0.0 {
        return BLACK;
    }

    let reflection_vector = (-light_vector).reflect_around(normal_vector);
    let reflection_dot_eye = dot(&reflection_vector, &eye_vector);

    if reflection_dot_eye < 0.0 {
        // This means the light reflects away from the eye...
        return BLACK;
    } else {
        let specular_factor = reflection_dot_eye.powf(*material.shininess());
        return *light.intensity() * *material.specular() * specular_factor;
    }
}
