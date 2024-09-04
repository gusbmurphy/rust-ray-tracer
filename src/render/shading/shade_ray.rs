use crate::prelude::*;
use crate::render::shading::ambient::calculate_ambient_contribution;
use crate::render::shading::diffuse::calculate_diffuse_contribution;
use crate::render::shading::reflective::calculate_reflective_contribution;
use crate::render::shading::specular::calculate_specular_contribution;

pub fn shade_ray(world: &World, ray: &Ray) -> Color {
    shade_ray_with_maximum_recursion(world, ray, 0)
}

const MAX_RECURSION: i8 = 5;

pub fn shade_ray_with_maximum_recursion(
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

// This adjusts the hit so that it's ever so slightly on the outside of the intersected shape.
pub fn adjust_hit(hit: &Intersection) -> Point {
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
        let pattern = StripePattern::new(WHITE, BLACK);

        let material = MaterialBuilder::new()
            .specular(0.0) // Setting ambient to 1.0 to simplify the color of any hit...
            .diffuse(0.0)
            .ambient(1.0)
            .pattern(Box::new(pattern))
            .build();

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
        let mut pattern = StripePattern::new(WHITE, GREEN);
        pattern.set_transform(Transform::translation(1.0, 0.0, 0.0));

        let material = MaterialBuilder::new()
            .specular(0.0)
            .diffuse(0.0)
            .ambient(1.0) // Setting ambient to 1.0 to simplify the color of any hit...
            .pattern(Box::new(pattern))
            .build();

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
        let pattern = StripePattern::new(WHITE, GREEN);
        let material = MaterialBuilder::new()
            .specular(0.0)
            .diffuse(0.0)
            .ambient(1.0)
            .pattern(Box::new(pattern))
            .build();

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
        let pattern = GradientPattern::new(WHITE, GREEN);
        let material = MaterialBuilder::new()
            .specular(0.0)
            .diffuse(0.0)
            .ambient(1.0) // Setting ambient to 1.0 to simplify the color of any hit...
            .pattern(Box::new(pattern))
            .build();

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
        let mut pattern = RingPattern::new(WHITE, GREEN);
        // Scaling it down so we see a few rings...
        pattern.set_transform(Transform::scaling(0.25, 0.25, 0.25));

        let material = MaterialBuilder::new()
            .specular(0.0)
            .diffuse(0.0)
            .ambient(1.0) // Setting ambient to 1.0 to simplify the color of any hit...
            .pattern(Box::new(pattern))
            .build();

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
        let first_sphere_material = MaterialBuilder::new()
            .flat_color(Color::new(0.8, 1.0, 0.6))
            .specular(0.2)
            .diffuse(0.7)
            .ambient(1.0)
            .build();

        let mut first_sphere = Sphere::new();
        first_sphere.set_material(first_sphere_material);

        let second_sphere_material = MaterialBuilder::new().ambient(1.0).build();

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
        let plane_material = MaterialBuilder::new().reflective(0.5).build();
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

        let material_a = MaterialBuilder::new().reflective(1.0).build();
        let material_b = MaterialBuilder::new().reflective(1.0).build();

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
