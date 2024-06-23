use crate::prelude::*;

// TODO: How about this takes the world and a ray? Then we can just figure out the intersection?
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

    #[test]
    fn ray_between_light_and_sphere() {
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let sphere = Sphere::new();

        let mut world = World::new();
        world.set_light(light);
        world.add_sphere(sphere);

        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), POSITIVE_Z);
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let result = shade_hit(&world, &intersection);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn eye_between_light_and_point_and_eye_offset_45_degrees() {
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let sphere = Sphere::new();

        let mut world = World::new();
        world.set_light(light);
        world.add_sphere(sphere);

        let ray = Ray::new(Point::new(0.0, 5.0, -5.0), Vector::new(0.0, -2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0));
        let intersection = Intersection::new(4.0, &sphere, &ray);

        let result = shade_hit(&world, &intersection);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    // #[test]
    // fn eye_between_light_and_point_and_light_offset_45_degrees() {
    //     let material = Material::new();
    //     let point = Point::new(0.0, 0.0, 0.0);

    //     let calculator = LightingCalculator {
    //         eye_vector: Vector::new(0.0, 0.0, -1.0),
    //         normal_vector: Vector::new(0.0, 0.0, -1.0),
    //         light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0)),
    //     };

    //     let result = calculator.color_for_material_at(material, point, false);

    //     assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    // }

    // #[test]
    // fn eye_in_path_of_reflection_vector() {
    //     let material = Material::new();
    //     let point = Point::new(0.0, 0.0, 0.0);

    //     let calculator = LightingCalculator {
    //         eye_vector: Vector::new(0.0, -2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0),
    //         normal_vector: Vector::new(0.0, 0.0, -1.0),
    //         light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0)),
    //     };

    //     let result = calculator.color_for_material_at(material, point, false);

    //     assert_eq!(result, Color::new(1.63638, 1.63638, 1.63638));
    // }

    // #[test]
    // fn light_on_opposite_side_of_surface() {
    //     let material = Material::new();
    //     let point = Point::new(0.0, 0.0, 0.0);

    //     let calculator = LightingCalculator {
    //         eye_vector: Vector::new(0.0, 0.0, -1.0),
    //         normal_vector: Vector::new(0.0, 0.0, -1.0),
    //         light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0)),
    //     };

    //     let result = calculator.color_for_material_at(material, point, false);

    //     assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    // }

    // #[test]
    // fn when_the_surface_is_in_a_shadow_we_only_use_the_ambient_component() {
    //     let material = Material::new();
    //     let point = Point::new(0.0, 0.0, 0.0);

    //     let calculator = LightingCalculator {
    //         eye_vector: Vector::new(0.0, 0.0, -1.0),
    //         normal_vector: Vector::new(0.0, 0.0, -1.0),
    //         light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0)),
    //     };

    //     let result = calculator.color_for_material_at(material, point, true);

    //     assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    // }
}
