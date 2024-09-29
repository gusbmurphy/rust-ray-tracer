use std::error::Error;
use std::rc::Rc;

use crate::{parse::parse_little_things::parse_values, prelude::*};
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use super::parse_little_things::{parse_color, parse_f64_from_integer_or_real};

pub fn parse_shape(
    map: Option<&LinkedHashMap<Yaml, Yaml>>,
    shape_name: &str,
) -> Result<Rc<dyn Shape>, Box<dyn Error>> {
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
                "material" => given_material = Some(parse_material(value)?),
                "transform" => given_transform = Some(parse_transform(value)?),
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

    Ok(Rc::from(shape))
}

fn parse_material(yaml: &Yaml) -> Result<Material, Box<dyn Error>> {
    let map = yaml.as_hash().unwrap();

    let mut material_builder = MaterialBuilder::new();

    for (key, value) in map {
        match key.as_str().unwrap() {
            "pattern" => {
                if let Ok(pattern) = parse_pattern(value) {
                    material_builder = material_builder.pattern(pattern);
                }
            }
            "diffuse" => {
                if let Ok(diffuse) = parse_f64_from_integer_or_real(value) {
                    material_builder = material_builder.diffuse(diffuse);
                }
            }
            "specular" => {
                if let Ok(specular) = parse_f64_from_integer_or_real(value) {
                    material_builder = material_builder.specular(specular);
                }
            }
            "shininess" => {
                if let Ok(shininess) = parse_f64_from_integer_or_real(value) {
                    material_builder = material_builder.shininess(shininess);
                }
            }
            "ambient" => {
                if let Ok(ambient) = parse_f64_from_integer_or_real(value) {
                    material_builder = material_builder.ambient(ambient);
                }
            }
            "reflective" => {
                if let Ok(reflective) = parse_f64_from_integer_or_real(value) {
                    material_builder = material_builder.reflective(reflective);
                }
            }
            "transparency" => {
                if let Ok(transparency) = parse_f64_from_integer_or_real(value) {
                    material_builder = material_builder.transparency(transparency);
                }
            }
            "refractive_index" => {
                if let Ok(refractive_index) = parse_f64_from_integer_or_real(value) {
                    material_builder = material_builder.refractive_index(refractive_index);
                }
            }
            _ => todo!(),
        }
    }

    Ok(material_builder.build())
}

fn parse_pattern(yaml: &Yaml) -> Result<Box<dyn Pattern>, Box<dyn Error>> {
    if let Some(_map) = yaml.as_hash() {
        parse_single_pattern(yaml)
    } else {
        let mut patterns: Vec<Rc<dyn Pattern>> = Vec::new();

        for node in yaml.as_vec().unwrap() {
            let pattern = parse_single_pattern(node).unwrap();
            patterns.push(Rc::from(pattern));
        }

        Ok(Box::new(BlendedPattern::new(patterns)))
    }
}

fn parse_single_pattern(yaml: &Yaml) -> Result<Box<dyn Pattern>, Box<dyn Error>> {
    let mut pattern: Option<Box<dyn Pattern>> = None;

    for (key, value) in yaml.as_hash().unwrap() {
        match key.as_str().unwrap() {
            "flat" => {
                let color = parse_color(value)?;
                pattern = Some(Box::new(FlatPattern::new(color)));
            }
            "stripes" => {
                pattern = Some(parse_transformable_pattern(value, |colors: [Color; 2]| {
                    Box::new(StripePattern::new(colors[0], colors[1]))
                })?);
            }
            "gradient" => {
                pattern = Some(parse_transformable_pattern(value, |colors: [Color; 2]| {
                    Box::new(GradientPattern::new(colors[0], colors[1]))
                })?);
            }
            "checkers" => {
                pattern = Some(parse_checkers(value)?);
            }
            "rings" => {
                pattern = Some(parse_transformable_pattern(value, |colors: [Color; 2]| {
                    Box::new(RingPattern::new(colors[0], colors[1]))
                })?);
            }
            _ => todo!(),
        }
    }

    Ok(pattern.unwrap())
}

fn parse_checkers(yaml: &Yaml) -> Result<Box<dyn Pattern>, Box<dyn Error>> {
    if !yaml["subpatterns"].is_badvalue() {
        let subpattern_nodes = yaml["subpatterns"].as_vec().unwrap();

        let pattern_a = parse_single_pattern(subpattern_nodes.get(0).unwrap())?;
        let pattern_b = parse_single_pattern(subpattern_nodes.get(1).unwrap())?;

        Ok(Box::new(Checker3DPattern::new_with_patterns(
            pattern_a, pattern_b,
        )))
    } else {
        parse_transformable_pattern(yaml, |colors: [Color; 2]| {
            Box::new(Checker3DPattern::new(colors[0], colors[1]))
        })
    }
}

fn parse_transformable_pattern<F>(
    value: &Yaml,
    constructor: F,
) -> Result<Box<dyn Pattern>, Box<dyn Error>>
where
    F: Fn([Color; 2]) -> Box<dyn Pattern>,
{
    let (colors, transform) = parse_pattern_values(value)?;

    let mut pattern = constructor(colors);

    if let Some(t) = transform {
        pattern.set_transform(t)
    }

    Ok(pattern)
}

fn parse_pattern_values(value: &Yaml) -> Result<([Color; 2], Option<Transform>), Box<dyn Error>> {
    let colors = parse_color_pair(value)?;

    let mut transform: Option<Transform> = None;
    let transform_yaml = &value["transform"];
    if !transform_yaml.is_badvalue() {
        transform = parse_transform(&value["transform"]).ok();
    }

    Ok((colors, transform))
}

fn parse_color_pair(yaml: &Yaml) -> Result<[Color; 2], Box<dyn Error>> {
    let value_vec = yaml["colors"].as_vec().unwrap().to_owned();

    let first_color = parse_color(value_vec.get(0).unwrap())?;
    let second_color = parse_color(value_vec.get(1).unwrap())?;

    Ok([first_color, second_color])
}

fn parse_transform(yaml: &Yaml) -> Result<Transform, Box<dyn Error>> {
    let nodes = yaml.as_vec().unwrap();

    let mut transform = Transform::new(IDENTITY_MATRIX);

    for node in nodes {
        let map = node.as_hash().unwrap();

        for (key, value) in map {
            match key.as_str().unwrap() {
                "translate" => {
                    let translation = parse_translation(value);
                    transform = transform * translation;
                }
                "scale" => {
                    let scaling = parse_scaling(value);
                    transform = transform * scaling;
                }
                "rotate_x" => {
                    let radians = parse_f64_from_integer_or_real(value)?;
                    let rotation = Transform::x_rotation(radians);
                    transform = transform * rotation;
                }
                "rotate_y" => {
                    let radians = parse_f64_from_integer_or_real(value)?;
                    let rotation = Transform::y_rotation(radians);
                    transform = transform * rotation;
                }
                "rotate_z" => {
                    let radians = parse_f64_from_integer_or_real(value)?;
                    let rotation = Transform::z_rotation(radians);
                    transform = transform * rotation;
                }
                "shear" => {
                    let shearing = parse_shearing(value)?;
                    transform = transform * shearing;
                }
                _ => todo!(),
            }
        }
    }

    Ok(transform)
}

fn parse_translation(yaml: &Yaml) -> Transform {
    let values = parse_values(yaml.as_vec().unwrap().to_owned()).unwrap();
    Transform::translation(values[0].unwrap(), values[1].unwrap(), values[2].unwrap())
}

fn parse_scaling(yaml: &Yaml) -> Transform {
    let values = parse_values(yaml.as_vec().unwrap().to_owned()).unwrap();
    Transform::scaling(values[0].unwrap(), values[1].unwrap(), values[2].unwrap())
}

fn parse_shearing(yaml: &Yaml) -> Result<Transform, Box<dyn Error>> {
    let mut shear_values = [0.0f64; 6];
    let value_vec = yaml.as_vec().unwrap();

    for (i, value) in value_vec.iter().enumerate() {
        shear_values[i] = parse_f64_from_integer_or_real(value)?;
    }

    Ok(Transform::shearing(
        shear_values[0],
        shear_values[1],
        shear_values[2],
        shear_values[3],
        shear_values[4],
        shear_values[5],
    ))
}
