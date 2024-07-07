use crate::prelude::*;

pub fn shade_ray(world: &World, ray: &Ray) -> Color {
    if let Some(hit) = world.hit_for(ray) {
        shade_hit(world, &hit)
    } else {
        BLACK
    }
}

fn shade_hit(world: &World, hit: &Intersection) -> Color {
    let eye_vector = -hit.ray().direction().to_owned();

    let adjusted_hit = adjust_hit(&hit);
    let hit_is_in_shadow = world.is_point_shadowed(&adjusted_hit);

    let material = hit.material();
    let light = world.light();

    let hit_in_object_space = hit.object().transform().invert().unwrap() * adjusted_hit;
    let effective_color = material.color_at(&hit_in_object_space) * *light.intensity();

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
            light_vector,
            &hit.normal_vector(),
            &eye_vector,
            hit.material(),
            light,
        );
    }

    return ambient_contribution + diffuse_contribution + specular_contribution;
}

fn calculate_specular_contribution(
    light_vector: Vector,
    normal_vector: &Vector,
    eye_vector: &Vector,
    material: &Material,
    light: &PointLight,
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

fn adjust_hit(hit: &Intersection) -> Point {
    hit.point() + hit.normal_vector() * EPSILON
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn shading_a_ray() {
        let world = World::create_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let result = shade_ray(&world, &ray);

        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::create_default();

        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.25, 0.0));
        world.set_light(light);

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let result = shade_ray(&world, &ray);

        assert_eq!(result, Color::new(0.90498, 0.90498, 0.90498))
    }

    #[test]
    fn color_when_ray_misses_everything() {
        let world = World::create_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

        let result = shade_ray(&world, &ray);

        assert_eq!(result, BLACK);
    }

    #[test]
    fn color_for_a_ray_that_hits() {
        let world = World::create_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let result = shade_ray(&world, &ray);

        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_for_a_ray_hitting_a_sphere_with_a_striped_pattern() {
        let mut material = Material::new();
        // Setting ambient to 1.0 to simplify the color of any hit...
        material.set_specular(0.0);
        material.set_diffuse(0.0);
        material.set_ambient(1.0);

        let pattern = StripePattern::new(WHITE, BLACK);
        material.set_pattern(Box::new(pattern));

        let mut sphere = Sphere::new();
        sphere.set_material(material);

        let mut world = World::new();
        world.add_shape(Rc::new(sphere));

        let ray_hitting_black = Ray::new(Point::new(-0.1, 0.0, -5.0), POSITIVE_Z);
        let ray_hitting_white = Ray::new(Point::new(0.1, 0.0, -5.0), POSITIVE_Z);

        assert_eq!(shade_ray(&world, &ray_hitting_black), BLACK);
        assert_eq!(shade_ray(&world, &ray_hitting_white), WHITE);
    }

    #[test]
    fn hits_on_a_sphere_with_a_transform_and_stripes_with_their_own_transform() {
        let mut material = Material::new();
        // Setting ambient to 1.0 to simplify the color of any hit...
        material.set_specular(0.0);
        material.set_diffuse(0.0);
        material.set_ambient(1.0);

        let mut pattern = StripePattern::new(WHITE, GREEN);
        pattern.set_transform(Transform::translation(1.0, 0.0, 0.0));
        material.set_pattern(Box::new(pattern));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        sphere.set_material(material);

        let mut world = World::new();
        world.add_shape(Rc::new(sphere));

        let ray = Ray::new(Point::new(1.5, 0.0, 0.0), POSITIVE_Z);

        assert_eq!(shade_ray(&world, &ray), GREEN);
    }

    #[test]
    fn hits_on_a_sphere_with_a_transform_and_stripes() {
        let mut material = Material::new();
        // Setting ambient to 1.0 to simplify the color of any hit...
        material.set_specular(0.0);
        material.set_diffuse(0.0);
        material.set_ambient(1.0);

        let pattern = StripePattern::new(WHITE, GREEN);
        material.set_pattern(Box::new(pattern));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        sphere.set_material(material);

        let mut world = World::new();
        world.add_shape(Rc::new(sphere));

        let ray = Ray::new(Point::new(1.5, 0.0, 0.0), POSITIVE_Z);

        assert_eq!(shade_ray(&world, &ray), WHITE);
    }

    #[test]
    fn hits_on_a_sphere_with_a_gradient_pattern() {
        let mut material = Material::new();
        // Setting ambient to 1.0 to simplify the color of any hit...
        material.set_specular(0.0);
        material.set_diffuse(0.0);
        material.set_ambient(1.0);

        let pattern = GradientPattern::new(WHITE, GREEN);
        material.set_pattern(Box::new(pattern));

        let mut sphere = Sphere::new();
        sphere.set_material(material);

        let mut world = World::new();
        world.add_shape(Rc::new(sphere));

        let ray_at_start = Ray::new(Point::new(-0.5, 0.0, 0.0), POSITIVE_Z);
        let ray_at_end = Ray::new(Point::new(0.5, 0.0, 0.0), POSITIVE_Z);
        assert_eq!(shade_ray(&world, &ray_at_start), WHITE);
        assert_eq!(shade_ray(&world, &ray_at_end), GREEN);
    }

    #[test]
    fn color_for_a_ray_that_hits_but_originates_inside_a_different_object() {
        // Setting the ambient value of each sphere's material to 1 to simplify things...
        let mut first_sphere_material = Material::new();
        first_sphere_material.set_flat_color(Color::new(0.8, 1.0, 0.6));
        first_sphere_material.set_specular(0.2);
        first_sphere_material.set_diffuse(0.7);
        first_sphere_material.set_ambient(1.0);

        let mut first_sphere = Sphere::new();
        first_sphere.set_material(first_sphere_material);

        let mut second_sphere_material = Material::new();
        second_sphere_material.set_ambient(1.0);

        let second_sphere_scaling = Transform::scaling(0.5, 0.5, 0.5);

        let mut second_sphere = Sphere::new();
        second_sphere.set_transform(second_sphere_scaling);
        second_sphere.set_material(second_sphere_material);

        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

        let mut world = World::new();
        world.add_sphere(first_sphere);
        world.add_sphere(second_sphere);
        world.set_light(light);

        // This ray originates inside of the outermost sphere, and is pointed at the inner one.
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let result = shade_ray(&world, &ray);

        // Since the ambient is 1, the color will just be the color of that inner sphere.
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn getting_the_color_for_a_shaded_hit() {
        let mut world = World::new();
        world.set_light(PointLight::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new(0.0, 0.0, -10.0),
        ));

        let sphere_one = Sphere::new();
        world.add_sphere(sphere_one);

        let mut sphere_two = Sphere::new();
        sphere_two.set_transform(Transform::translation(0.0, 0.0, -5.0));
        world.add_sphere(sphere_two);

        let ray = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0));

        let result = shade_ray(&world, &ray);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
