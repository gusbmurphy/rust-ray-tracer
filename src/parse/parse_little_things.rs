use core::fmt;
use std::error::Error;

use crate::prelude::*;
use yaml_rust::{yaml, Yaml};

pub fn parse_point(yaml_array: yaml::Array) -> Result<Point, Box<dyn Error>> {
    let values = parse_values(yaml_array)?;

    Ok(Point::new(
        values[0].unwrap(),
        values[1].unwrap(),
        values[2].unwrap(),
    ))
}

pub fn parse_vector(yaml_array: yaml::Array) -> Result<Vector, Box<dyn Error>> {
    let values = parse_values(yaml_array)?;

    Ok(Vector::new(
        values[0].unwrap(),
        values[1].unwrap(),
        values[2].unwrap(),
    ))
}

pub fn parse_color(yaml: &Yaml) -> Result<Color, Box<dyn Error>> {
    let yaml_array = yaml.as_vec().unwrap().to_owned();
    let values = parse_values(yaml_array)?;

    Ok(Color::new(
        values[0].unwrap(),
        values[1].unwrap(),
        values[2].unwrap(),
    ))
}

pub fn parse_values(yaml_array: yaml::Array) -> Result<[Option<f64>; 3], Box<dyn Error>> {
    let mut values: [Option<f64>; 3] = [None; 3];

    for (index, yaml) in yaml_array.iter().enumerate() {
        match yaml {
            Yaml::Integer(_) => values[index] = Some(parse_f64_from_integer_or_real(yaml)?),
            Yaml::Real(_) => values[index] = Some(parse_f64_from_integer_or_real(yaml)?),
            _ => values[index] = None,
        }
    }

    Ok(values)
}

pub fn parse_f64_from_integer_or_real(yaml: &Yaml) -> Result<f64, Box<dyn Error>> {
    match yaml {
        Yaml::Integer(ref i) => Ok(i.to_owned() as f64),
        Yaml::Real(ref r) => Ok(r.parse::<f64>()?),
        _ => Err(Box::new(YamlParsingError::new(
            "Expected to find numeric value",
        ))),
    }
}

#[derive(Debug)]
pub struct YamlParsingError {
    message: String,
}

impl YamlParsingError {
    pub fn new(message: &str) -> Self {
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
