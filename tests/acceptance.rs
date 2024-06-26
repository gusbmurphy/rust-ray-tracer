use core::f32::consts::PI;

use ray_tracer::geometry::*;
use ray_tracer::physical::*;
use ray_tracer::render::*;

#[test]
fn just_three_spheres() {
    let mut world = World::new();
    world.add_sphere(create_middle_sphere());
    world.add_sphere(create_right_sphere());
    world.add_sphere(create_left_sphere());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
    world.set_light(light);

    let camera_transform = Transform::new_view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let camera =
        Camera::new_with_transform(100, 50, PI / 3.0, camera_transform);

    insta::assert_yaml_snapshot!(create_ppm_from_canvas(camera.render(world)));
}

fn create_middle_sphere() -> Sphere {
    let mut sphere = Sphere::new();
    sphere.set_transform(Transform::new_translation(-0.5, 1.0, 0.5));

    let mut material = Material::new();
    material.set_color(Color::new(0.1, 1.0, 0.5));
    material.set_diffuse(0.7);
    material.set_specular(0.3);

    sphere.set_material(material);

    sphere
}

fn create_right_sphere() -> Sphere {
    let mut sphere = Sphere::new();
    sphere.set_transform(
        Transform::new_translation(1.5, 1.0, -0.5) * Transform::new_scaling(0.5, 0.5, 0.5),
    );

    let mut material = Material::new();
    material.set_color(Color::new(0.5, 1.0, 0.1));
    material.set_diffuse(0.7);
    material.set_specular(0.3);

    sphere.set_material(material);

    sphere
}

fn create_left_sphere() -> Sphere {
    let mut sphere = Sphere::new();
    sphere.set_transform(
        Transform::new_translation(-1.5, 0.33, -0.75) * Transform::new_scaling(0.33, 0.33, 0.33),
    );

    let mut material = Material::new();
    material.set_color(Color::new(1.0, 0.8, 0.1));
    material.set_diffuse(0.7);
    material.set_specular(0.3);

    sphere.set_material(material);

    sphere
}
