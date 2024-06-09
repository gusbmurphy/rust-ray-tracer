use crate::prelude::*;
use core::f32::consts::PI;

pub fn draw_scene_ppm() -> String {
    let mut world = World::new();

    let mut floor_material = Material::new();
    floor_material.set_specular(0.0);
    floor_material.set_color(Color::new(1.0, 0.9, 0.9));

    world.add_sphere(create_floor(floor_material));
    world.add_sphere(create_left_wall(floor_material));
    world.add_sphere(create_right_wall(floor_material));
    world.add_sphere(create_middle_sphere());
    world.add_sphere(create_left_sphere());
    world.add_sphere(create_right_sphere());

    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));
    world.set_light(light);

    let camera_transform = Transform::new_view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let camera = Camera::new_with_transform(100, 50, PI/3.0, camera_transform);

    create_ppm_from_canvas(camera.render(world))
}

fn create_floor(material: Material) -> Sphere {
    let mut floor = Sphere::new();
    floor.set_transform(Transform::new_scaling(10.0, 0.01, 10.0));
    floor.set_material(material);

    floor
}

fn create_left_wall(material: Material) -> Sphere {
    let mut wall = Sphere::new();

    let transform = Transform::new_translation(0.0, 0.0, 5.0)
        * Transform::new_y_rotation(-PI / 4.0)
        * Transform::new_x_rotation(PI / 2.0)
        * Transform::new_scaling(10.0, 0.01, 10.0);
    wall.set_transform(transform);

    wall.set_material(material);

    wall
}

fn create_right_wall(material: Material) -> Sphere {
    let mut wall = Sphere::new();

    let transform = Transform::new_translation(0.0, 0.0, 5.0)
        * Transform::new_y_rotation(PI / 4.0)
        * Transform::new_x_rotation(PI / 2.0)
        * Transform::new_scaling(10.0, 0.01, 10.0);
    wall.set_transform(transform);

    wall.set_material(material);

    wall
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
