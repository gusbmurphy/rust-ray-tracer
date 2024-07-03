use crate::parse::parse_little_things::*;
use crate::prelude::*;

use linked_hash_map::LinkedHashMap;
use std::error::Error;
use yaml_rust::Yaml;

pub fn parse_light(map: &LinkedHashMap<Yaml, Yaml>) -> Result<PointLight, Box<dyn Error>> {
    let mut at: Option<Point> = None;
    let mut intensity: Option<Color> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "at" => at = Some(parse_point(value.as_vec().unwrap().to_owned()).unwrap()),
            "intensity" => {
                intensity = Some(parse_color(value.as_vec().unwrap().to_owned()).unwrap())
            }
            _ => todo!(),
        }
    }

    Ok(PointLight::new(intensity.unwrap(), at.unwrap()))
}
