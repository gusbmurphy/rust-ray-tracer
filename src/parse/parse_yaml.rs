use std::{error::Error, fs::read_to_string};

use yaml_rust::YamlLoader;

use crate::{
    parse::{parse_camera::parse_camera, parse_light::parse_light, parse_shape::parse_shape},
    prelude::*,
};

pub fn parse_scene_from_yaml(file_path: &str) -> Result<(World, Camera), Box<dyn Error>> {
    let text = read_to_string(file_path)?;
    let yaml = YamlLoader::load_from_str(text.as_str())?;

    let mut world = World::new();
    let mut camera = Camera::new(100, 100, 100.0);

    for node in &yaml {
        match node {
            yaml_rust::Yaml::Hash(ref h) => {
                for (key, value) in h {
                    let value_hash = value.as_hash().unwrap();
                    match key.as_str().unwrap() {
                        "camera" => camera = parse_camera(value_hash)?,
                        "light" => world.set_light(parse_light(value_hash)?),
                        "sphere" => world.add_shape(parse_shape(value_hash)?),
                        _ => todo!(),
                    }
                    println!("{:?}:", key);
                }
            }
            _ => {
                todo!();
            }
        }
    }

    return Ok((world, camera));
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;

    #[test]
    fn parsing_a_scene_with_a_sphere() {
        let (world, camera) =
            parse_scene_from_yaml("src/parse/examples/scene_with_sphere.yaml").unwrap();

        let light = world.light().unwrap();
        assert_eq!(light.intensity().to_owned(), Color::new(1.0, 1.0, 1.0));

        let shapes = world.shapes().to_owned();
        assert_eq!(shapes.len(), 1);

        let only_shape = shapes.get(0).unwrap();
        let some_sphere = Sphere::new();
        assert_eq!(only_shape.type_id(), some_sphere.type_id());

        let mut expected_material = Material::new();
        expected_material.set_diffuse(0.7);
        expected_material.set_specular(0.3);
        expected_material.set_color(Color::new(0.1, 1.0, 0.5));
        assert_eq!(only_shape.material().to_owned(), expected_material);

        assert_eq!(camera.width().to_owned(), 100);
        assert_eq!(camera.height().to_owned(), 100);
    }
}
