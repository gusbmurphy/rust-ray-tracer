use core::fmt;
use std::error::Error;

use crate::prelude::*;
use yaml_rust::{yaml, Yaml};

pub fn parse_point(yaml_array: yaml::Array) -> Result<Point, Box<dyn Error>> {
    let values = parse_values(yaml_array)?;

    Ok(Point::new(
        values[0].unwrap(),
        values[0].unwrap(),
        values[0].unwrap(),
    ))
}

pub fn parse_vector(yaml_array: yaml::Array) -> Result<Vector, Box<dyn Error>> {
    let values = parse_values(yaml_array)?;

    Ok(Vector::new(
        values[0].unwrap(),
        values[0].unwrap(),
        values[0].unwrap(),
    ))
}

pub fn parse_color(yaml_array: yaml::Array) -> Result<Color, Box<dyn Error>> {
    let values = parse_values(yaml_array)?;

    Ok(Color::new(
        values[0].unwrap(),
        values[0].unwrap(),
        values[0].unwrap(),
    ))
}

pub fn parse_values(yaml_array: yaml::Array) -> Result<[Option<f32>; 3], Box<dyn Error>> {
    let mut values: [Option<f32>; 3] = [None; 3];

    for (index, yaml) in yaml_array.iter().enumerate() {
        match yaml {
            Yaml::Integer(ref i) => values[index] = Some(parse_f32_from_integer_or_real(yaml)?),
            Yaml::Real(ref r) => values[index] = Some(parse_f32_from_integer_or_real(yaml)?),
            _ => values[index] = None,
        }
    }

    Ok(values)
}

pub fn parse_f32_from_integer_or_real(yaml: &Yaml) -> Result<f32, Box<dyn Error>> {
    match yaml {
        Yaml::Integer(ref i) => Ok(i.to_owned() as f32),
        Yaml::Real(ref r) => Ok(r.parse::<f32>()?),
        _ => Err(Box::new(YamlParsingError::new(
            "Expected to find numeric value",
        ))),
    }
}

#[derive(Debug)]
struct YamlParsingError {
    message: String,
}

impl YamlParsingError {
    fn new(message: &str) -> Self {
        YamlParsingError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for YamlParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for YamlParsingError {}
