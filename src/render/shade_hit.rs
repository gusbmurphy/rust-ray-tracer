use crate::prelude::*;

pub fn shade_hit(world: &World, hit: &Intersection<Sphere>) -> Color {
    let eye_vector = -hit.ray().direction().to_owned();

    let adjusted_hit = adjust_hit(hit);
    let hit_is_in_shadow = world.is_point_shadowed(&adjusted_hit);

    let material = hit.intersected_object().material();
    let light = world.light().unwrap();

    let effective_color = *material.color() * *light.intensity();

    let light_vector = (*light.position() - hit.point()).normalize();

    let light_dot_normal = dot(&light_vector, &hit.normal_vector());

    let ambient_contribution = effective_color * material.ambient();

    if hit_is_in_shadow {
        return ambient_contribution;
    }

    let diffuse_contribution: Color;
    let specular_contribution: Color;

    if light_dot_normal < 0.0 {
        // This means the light is opposite the normal vector...
        diffuse_contribution = BLACK;
        specular_contribution = BLACK;
    } else {
        diffuse_contribution = effective_color * *material.diffuse() * light_dot_normal;
        specular_contribution = calculate_specular_contribution(
            light_vector, &hit.normal_vector(), &eye_vector, *hit.intersected_object().material(), light
        );
    }

    return ambient_contribution + diffuse_contribution + specular_contribution;
}

fn calculate_specular_contribution(
    light_vector: Vector,
    normal_vector: &Vector,
    eye_vector: &Vector,
    material: Material,
    light: PointLight
) -> Color {
    let reflection_vector = (-light_vector).reflect_around(normal_vector);
    let reflection_dot_eye = dot(&reflection_vector, eye_vector);

    if reflection_dot_eye < 0.0 {
        // This means the light reflects away from the eye...
        return BLACK;
    } else {
        let specular_factor = reflection_dot_eye.powf(*material.shininess());
        return *light.intensity() * *material.specular() * specular_factor;
    }
}

fn adjust_hit(hit: &Intersection<Sphere>) -> Point {
    hit.point() + hit.normal_vector() * EPSILON
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shading_an_intersection() {
        let world = World::create_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let hit = ray.cast_into(&world).unwrap();

        let result = shade_hit(&world, &hit);

        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::create_default();

        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.25, 0.0));
        world.set_light(light);

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let hit = ray.cast_into(&world).unwrap();

        let result = shade_hit(&world, &hit);

        assert_eq!(result, Color::new(0.90498, 0.90498, 0.90498))
    }
}
