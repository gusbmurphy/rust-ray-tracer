use crate::parse::parse_little_things::*;
use std::error::Error;

use crate::prelude::*;
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

pub fn parse_camera(map: &LinkedHashMap<Yaml, Yaml>) -> Result<Camera, Box<dyn Error>> {
    let mut width: Option<u64> = None;
    let mut height: Option<u64> = None;
    let mut fov: Option<f32> = None;
    let mut from: Option<Point> = None;
    let mut to: Option<Point> = None;
    let mut up: Option<Vector> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "width" => width = Some(value.as_i64().unwrap() as u64),
            "height" => height = Some(value.as_i64().unwrap() as u64),
            "fov" => fov = Some(parse_f32_from_integer_or_real(value)?),
            "from" => from = Some(parse_point(value.as_vec().unwrap().to_owned()).unwrap()),
            "to" => to = Some(parse_point(value.as_vec().unwrap().to_owned()).unwrap()),
            "up" => up = Some(parse_vector(value.as_vec().unwrap().to_owned()).unwrap()),
            _ => todo!(),
        }
    }

    let transform = Transform::view(from.unwrap(), to.unwrap(), up.unwrap());

    Ok(Camera::new_with_transform(
        width.unwrap(),
        height.unwrap(),
        fov.unwrap(),
        transform,
    ))
}
