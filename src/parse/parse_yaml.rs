use std::{error::Error, fs::read_to_string};

use yaml_rust::YamlLoader;

use crate::{
    parse::{parse_camera::parse_camera, parse_light::parse_light, parse_shape::parse_shape},
    prelude::*,
};

use super::parse_little_things::parse_color;

pub fn parse_scene_from_yaml(file_path: &str) -> Result<(World, Camera), Box<dyn Error>> {
    let text = read_to_string(file_path)?;
    let nodes = YamlLoader::load_from_str(text.as_str())?
        .get(0)
        .unwrap()
        .as_vec()
        .unwrap()
        .to_owned();

    let mut world = World::new();
    let mut camera = Camera::new(100, 100, 100.0);
    let mut background = BLACK;

    for node in nodes {
        match node {
            yaml_rust::Yaml::Hash(h) => {
                for (key, value) in h {
                    match key.as_str().unwrap() {
                        "camera" => camera = parse_camera(value.as_hash().unwrap().clone())?,
                        "light" => world.set_light(parse_light(value.as_hash().unwrap())?),
                        "sphere" | "plane" => {
                            world.add_shape(parse_shape(value.as_hash(), key.as_str().unwrap())?)
                        }
                        "background" => background = parse_color(&value).unwrap(),
                        _ => todo!(),
                    }
                }
            }
            _ => {
                todo!();
            }
        }
    }

    world.set_background(background);

    return Ok((world, camera));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn a_light_is_correctly_parsed() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/scene_with_sphere.yaml").unwrap();

        let light = world.light();
        assert_eq!(light.intensity().to_owned(), Color::new(1.0, 1.0, 1.0));
        assert_eq!(light.position().to_owned(), Point::new(-10.0, 10.0, -10.0));
    }

    #[test]
    fn a_sphere_is_correctly_parsed() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/scene_with_sphere.yaml").unwrap();

        let shapes = world.shapes().to_owned();
        assert_eq!(shapes.len(), 1);

        let only_shape = shapes.get(0).unwrap().to_owned();
        assert_eq!(only_shape.shape_type(), ShapeType::Sphere);

        let expected_material = MaterialBuilder::new()
            .diffuse(0.7)
            .specular(0.3)
            .flat_color(Color::new(0.1, 1.0, 0.5))
            .build();
        assert_eq!(*only_shape.material(), expected_material);

        let expected_transform =
            Transform::translation(-0.5, 1.0, 0.5) * Transform::scaling(0.5, 0.5, 0.5);
        assert_eq!(*only_shape.transform(), expected_transform);
    }

    #[test]
    fn the_camera_is_correctly_parsed() {
        let (_world, camera) =
            parse_scene_from_yaml("tests/scenes/scene_with_sphere.yaml").unwrap();

        assert_eq!(camera.width().to_owned(), 100);
        assert_eq!(camera.height().to_owned(), 100);
        assert_eq!(camera.fov().to_owned(), 1.04719);

        let expected_transform = Transform::view(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        assert_eq!(*camera.transform(), expected_transform)
    }

    #[test]
    fn a_plane_is_correctly_parsed() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/scene_with_plane_and_sphere.yaml").unwrap();

        let shapes = world.shapes().to_owned();
        assert_eq!(shapes.len(), 2); // There is also a sphere here!

        let plane = shapes.get(1).unwrap().to_owned();
        assert_eq!(plane.shape_type(), ShapeType::Plane);

        let expected_material = MaterialBuilder::new()
            .diffuse(1.1)
            .specular(0.2)
            .flat_color(Color::new(0.8, 2.0, 10.0))
            .build();
        assert_eq!(*plane.material(), expected_material);

        let expected_transform =
            Transform::translation(0.0, 0.0, 2.0) * Transform::x_rotation(1.57079);
        assert_eq!(*plane.transform(), expected_transform);
    }

    #[test]
    fn a_scene_with_three_spheres_gets_parsed_correctly() {
        let (world, _camera) = parse_scene_from_yaml("tests/scenes/three_spheres.yaml").unwrap();

        let shapes = world.shapes().to_owned();
        assert_eq!(shapes.len(), 3);
    }

    #[test]
    fn a_plane_can_have_no_attributes() {
        let (world, _camera) = parse_scene_from_yaml("tests/scenes/default_plane.yaml").unwrap();

        let shapes = world.shapes().to_owned();
        let plane = shapes.get(0).unwrap();

        // The plane should just have the default transform and material...
        assert_eq!(*plane.transform(), Transform::new(IDENTITY_MATRIX));
        assert_eq!(*plane.material(), MaterialBuilder::new().build());
    }

    #[test]
    fn stripes_are_parsed() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/sphere_with_stripes.yaml").unwrap();

        let sphere = world.shapes().get(0).unwrap();
        let material = sphere.material();

        let expected_material = MaterialBuilder::new()
            .pattern(Box::new(StripePattern::new(
                Color::new(0.1, 1.0, 0.5),
                Color::new(0.5, 1.0, 0.1),
            )))
            .diffuse(0.7)
            .specular(0.3)
            .build();

        assert_eq!(*material, expected_material);
    }

    #[test]
    fn stripes_can_also_have_a_transform() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/sphere_with_stripes_and_transforms.yaml").unwrap();

        let sphere = world.shapes().get(0).unwrap();
        let material = sphere.material();

        let mut expected_pattern =
            StripePattern::new(Color::new(0.1, 1.0, 0.5), Color::new(0.5, 1.0, 0.1));
        expected_pattern.set_transform(Transform::scaling(0.25, 0.25, 0.25));

        let expected_material = MaterialBuilder::new()
            .pattern(Box::new(expected_pattern))
            .diffuse(0.7)
            .specular(0.3)
            .build();

        assert_eq!(*material, expected_material);
    }

    #[test]
    fn a_sphere_with_every_transformation_is_parsed_correctly() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/sphere_with_every_transform.yaml").unwrap();

        let sphere = world.shapes().get(0).unwrap();
        let transform = sphere.transform();

        let expected_transform = Transform::translation(-0.5, 1.0, 0.5)
            * Transform::scaling(0.5, 3.0, 0.5)
            * Transform::x_rotation(1.57079)
            * Transform::y_rotation(0.78539)
            * Transform::z_rotation(2.51327)
            * Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        assert_eq!(*transform, expected_transform);
    }

    #[test]
    fn the_gradient_pattern_is_parsed_correctly() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/sphere_with_gradient.yaml").unwrap();

        let sphere = world.shapes().get(0).unwrap();
        let material = sphere.material();

        let mut expected_pattern =
            GradientPattern::new(Color::new(0.1, 1.0, 0.1), Color::new(1.0, 0.0, 0.5));
        expected_pattern.set_transform(Transform::z_rotation(0.78539));

        let expected_material = MaterialBuilder::new()
            .pattern(Box::new(expected_pattern))
            .diffuse(0.7)
            .specular(0.3)
            .build();

        assert_eq!(*material, expected_material);
    }

    #[test]
    fn the_ring_pattern_is_parsed_correctly() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/sphere_with_rings.yaml").unwrap();

        let sphere = world.shapes().get(0).unwrap();
        let material = sphere.material();

        let mut expected_pattern =
            RingPattern::new(Color::new(0.1, 0.8, 0.0), Color::new(1.0, 0.1, 0.5));
        expected_pattern.set_transform(Transform::scaling(0.4, 1.0, 0.08));

        let expected_material = MaterialBuilder::new()
            .pattern(Box::new(expected_pattern))
            .diffuse(0.7)
            .specular(0.3)
            .build();

        assert_eq!(*material, expected_material);
    }

    #[test]
    fn the_checkered_patten_is_parsed_correctly() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/plane_with_checkers.yaml").unwrap();

        let plane = world.shapes().get(0).unwrap();
        let material = plane.material();

        let expected_pattern =
            Checker3DPattern::new(Color::new(1.0, 0.0, 0.0), Color::new(0.5, 1.0, 0.1));

        let expected_material = MaterialBuilder::new()
            .pattern(Box::new(expected_pattern))
            .diffuse(0.7)
            .specular(0.3)
            .build();

        assert_eq!(*material, expected_material);
    }

    #[test]
    fn multiple_patterns_on_a_material_are_parsed_into_a_blended_pattern() {
        let (world, _camera) = parse_scene_from_yaml("tests/scenes/blended_patterns.yaml").unwrap();

        let plane = world.shapes().get(0).unwrap();
        let material = plane.material();

        let mut gradient =
            GradientPattern::new(Color::new(0.1, 1.0, 0.1), Color::new(1.0, 0.1, 0.5));
        gradient.set_transform(Transform::z_rotation(0.78539));

        let mut stripes = StripePattern::new(Color::new(0.1, 1.0, 0.5), Color::new(0.5, 1.0, 0.1));
        stripes.set_transform(Transform::scaling(0.15, 0.15, 0.15));

        let expected_pattern = BlendedPattern::new(vec![Rc::new(gradient), Rc::new(stripes)]);

        let expected_material = MaterialBuilder::new()
            .pattern(Box::new(expected_pattern))
            .diffuse(0.9)
            .specular(0.7)
            .build();

        assert_eq!(*material, expected_material);
    }

    #[test]
    fn materials_can_have_shininess() {
        let (world, _camera) = parse_scene_from_yaml("tests/scenes/shininess.yaml").unwrap();

        let plane = world.shapes().get(0).unwrap();
        let material = plane.material();

        assert_eq!(*material.shininess(), 0.5)
    }

    #[test]
    fn materials_can_have_transparency() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/transparent_sphere.yaml").unwrap();

        let sphere = world.shapes().get(0).unwrap();
        let material = sphere.material();

        assert_eq!(*material.transparency(), 0.7)
    }

    #[test]
    fn materials_can_have_a_refractive_index() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/transparent_sphere.yaml").unwrap();

        let sphere = world.shapes().get(0).unwrap();
        let material = sphere.material();

        assert_eq!(*material.refractive_index(), 2.417)
    }

    #[test]
    fn the_checkered_pattern_can_have_subpatterns() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/floor_with_nested_checkers.yaml").unwrap();

        let mut subpattern_a = Box::new(StripePattern::new(BLACK, WHITE));
        subpattern_a.set_transform(Transform::y_rotation(0.78539));
        let mut subpattern_b = Box::new(StripePattern::new(WHITE, BLACK));
        subpattern_b.set_transform(Transform::y_rotation(-0.78539));
        let expected_pattern = Checker3DPattern::new_with_patterns(subpattern_a, subpattern_b);

        let expected_material = MaterialBuilder::new()
            .pattern(Box::new(expected_pattern))
            .build();

        let material = world.shapes().get(0).unwrap().material();

        assert_eq!(*material, expected_material);
    }

    #[test]
    fn you_can_set_the_background() {
        let (world, _camera) =
            parse_scene_from_yaml("tests/scenes/reflective_floor_with_bg_color.yaml").unwrap();

        let bg = *world.background();

        assert_eq!(bg, RED);
    }
}
