use crate::prelude::*;

pub fn shade_ray(world: &World, ray: &Ray) -> Color {
    shade_ray_with_maximum_recursion(world, ray, 0)
}

const MAX_RECURSION: i8 = 5;

fn shade_ray_with_maximum_recursion(
    world: &World,
    ray: &Ray,
    current_recursion_count: i8,
) -> Color {
    if current_recursion_count <= MAX_RECURSION {
        if let Some(hit) = world.hit_for(ray) {
            shade_hit(world, &hit, current_recursion_count)
        } else {
            BLACK
        }
    } else {
        BLACK
    }
}

fn shade_hit(world: &World, hit: &Intersection, current_recursion_count: i8) -> Color {
    let light = world.light();

    let ambient_contribution = calculate_ambient_contribution(light, hit);

    if world.is_point_shadowed(&adjust_hit(&hit)) {
        return ambient_contribution;
    }

    return ambient_contribution
        + calculate_diffuse_contribution(light, hit)
        + calculate_specular_contribution(light, hit)
        + calculate_reflective_contribution(hit, world, current_recursion_count);
}

fn calculate_specular_contribution(light: &PointLight, hit: &Intersection) -> Color {
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

fn calculate_reflective_contribution(
    hit: &Intersection,
    world: &World,
    current_recursion_count: i8,
) -> Color {
    let adjusted_hit = adjust_hit(&hit);
    let material = hit.material();

    let reflection_vector = hit.ray().direction().reflect_around(&hit.normal_vector());

    shade_ray_with_maximum_recursion(
        world,
        &Ray::new(adjusted_hit, reflection_vector),
        current_recursion_count + 1,
    ) * *material.reflective()
}

fn calculate_ambient_contribution(light: &PointLight, hit: &Intersection) -> Color {
    let adjusted_hit = adjust_hit(&hit);
    let hit_in_object_space = hit.object().transform().invert().unwrap() * adjusted_hit;

    let effective_color = hit.material().color_at(&hit_in_object_space) * *light.intensity();

    effective_color * hit.material().ambient()
}

fn calculate_diffuse_contribution(light: &PointLight, hit: &Intersection) -> Color {
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
    fn hits_on_a_sphere_with_a_ring_pattern() {
        let mut material = Material::new();
        // Setting ambient to 1.0 to simplify the color of any hit...
        material.set_specular(0.0);
        material.set_diffuse(0.0);
        material.set_ambient(1.0);

        let mut pattern = RingPattern::new(WHITE, GREEN);
        // Scaling it down so we see a few rings...
        pattern.set_transform(Transform::scaling(0.25, 0.25, 0.25));

        material.set_pattern(Box::new(pattern));

        let mut sphere = Sphere::new();
        sphere.set_material(material);

        let mut world = World::new();
        world.add_shape(Rc::new(sphere));

        let ray_at_center = Ray::new(Point::new(0.0, 2.0, 0.0), NEGATIVE_Y);
        let ray_on_ring_in_x = Ray::new(Point::new(0.25, 2.0, 0.0), NEGATIVE_Y);
        let ray_on_ring_in_z = Ray::new(Point::new(0.0, 2.0, 0.25), NEGATIVE_Y);

        assert_eq!(shade_ray(&world, &ray_at_center), WHITE);
        assert_eq!(shade_ray(&world, &ray_on_ring_in_x), GREEN);
        assert_eq!(shade_ray(&world, &ray_on_ring_in_z), GREEN);
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

    #[test]
    fn shading_a_hit_on_a_reflective_surface() {
        let mut world = World::create_default();

        let mut plane = Plane::new();
        let mut plane_material = Material::new();
        plane_material.set_reflective(0.5);
        plane.set_material(plane_material);
        plane.set_transform(Transform::translation(0.0, -1.0, 0.0));

        world.add_shape(Rc::new(plane));

        // This ray hits the plane we just added, which should reflect the green color of one of
        // the spheres.
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );

        let result = shade_ray(&world, &ray);

        assert_eq!(result, Color::new(0.87675, 0.92434, 0.82917))
    }

    #[test]
    fn two_reflective_surfaces_do_not_cause_the_program_to_fail() {
        let mut world = World::new();
        world.set_light(PointLight::new(WHITE, ORIGIN));

        let mut material_a = Material::new();
        let mut material_b = Material::new();
        material_a.set_reflective(1.0);
        material_b.set_reflective(1.0);

        let mut lower_plane = Plane::new();
        lower_plane.set_material(material_a);
        lower_plane.set_transform(Transform::translation(0.0, -1.0, 0.0));

        let mut upper_plane = Plane::new();
        upper_plane.set_material(material_b);
        upper_plane.set_transform(Transform::translation(0.0, 1.0, 0.0));

        world.add_shape(Rc::new(lower_plane));
        world.add_shape(Rc::new(upper_plane));

        shade_ray(&world, &Ray::new(ORIGIN, POSITIVE_Y));
        // No assertion here because we just shouldn't get a stack overflow...
    }
}
