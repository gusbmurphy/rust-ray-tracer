use std::error::Error;

use crate::{parse::parse_little_things::parse_values, prelude::*};
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use super::parse_little_things::{parse_color, parse_f32_from_integer_or_real};

pub fn parse_shape(
    map: Option<&LinkedHashMap<Yaml, Yaml>>,
    shape_name: &str,
) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let mut given_material: Option<Material> = None;
    let mut given_transform: Option<Transform> = None;

    let mut shape: Box<dyn Shape> = match shape_name {
        "sphere" => Box::new(Sphere::new()),
        "plane" => Box::new(Plane::new()),
        _ => todo!(),
    };

    if let Some(m) = map {
        for (key, value) in m {
            match key.as_str().unwrap() {
                "material" => given_material = Some(parse_material(value.as_hash().unwrap())?),
                "transform" => given_transform = Some(parse_transform(value.as_vec().unwrap())?),
                _ => todo!(),
            }
        }

        if let Some(material) = given_material {
            shape.set_material(material);
        }
        if let Some(transform) = given_transform {
            shape.set_transform(transform);
        }

    }

    Ok(shape)
}

fn parse_material(map: &LinkedHashMap<Yaml, Yaml>) -> Result<Material, Box<dyn Error>> {
    let mut color: Option<Color> = None;
    let mut diffuse: Option<f32> = None;
    let mut specular: Option<f32> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "color" => color = Some(parse_color(value.as_vec().unwrap().to_owned()).unwrap()),
            "diffuse" => diffuse = Some(parse_f32_from_integer_or_real(value)?),
            "specular" => specular = Some(parse_f32_from_integer_or_real(value)?),
            _ => todo!(),
        }
    }

    let mut material = Material::new();
    material.set_color(color.unwrap());
    material.set_diffuse(diffuse.unwrap());
    material.set_specular(specular.unwrap());

    Ok(material)
}

fn parse_transform(nodes: &Vec<Yaml>) -> Result<Transform, Box<dyn Error>> {
    let mut transform = Transform::new(IDENTITY_MATRIX);

    for node in nodes {
        let map = node.as_hash().unwrap();

        for (key, value) in map {
            match key.as_str().unwrap() {
                "translate" => {
                    let values = parse_values(value.as_vec().unwrap().to_owned()).unwrap();
                    let translation = Transform::new_translation(
                        values[0].unwrap(),
                        values[1].unwrap(),
                        values[2].unwrap(),
                    );
                    transform = transform * translation;
                }
                "scale" => {
                    let values = parse_values(value.as_vec().unwrap().to_owned()).unwrap();
                    let scaling = Transform::new_scaling(
                        values[0].unwrap(),
                        values[1].unwrap(),
                        values[2].unwrap(),
                    );
                    transform = transform * scaling;
                }
                "rotate_x" => {
                    let radians = parse_f32_from_integer_or_real(value)?;
                    let rotation = Transform::new_x_rotation(radians);
                    transform = transform * rotation;
                }
                _ => todo!(),
            }
        }
    }

    Ok(transform)
}
