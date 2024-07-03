use std::error::Error;

use crate::prelude::*;
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

pub fn parse_shape(map: &LinkedHashMap<Yaml, Yaml>) -> Result<Box<dyn Shape>, Box<dyn Error>> {
    let mut material: Option<Material> = None;
    let mut transform: Option<Transform> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "material" => material = Some(parse_material(value.as_hash().unwrap())?),
            "transform" => transform = Some(parse_transform(value.as_hash().unwrap())?),
            _ => todo!(),
        }
    }

    let mut sphere = Sphere::new();
    sphere.set_material(material.unwrap());
    sphere.set_transform(transform.unwrap());

    Ok(Box::new(sphere))
}

fn parse_material(map: &LinkedHashMap<Yaml, Yaml>) -> Result<Material, Box<dyn Error>> {
    todo!()
}

fn parse_transform(map: &LinkedHashMap<Yaml, Yaml>) -> Result<Transform, Box<dyn Error>> {
    todo!()
}
