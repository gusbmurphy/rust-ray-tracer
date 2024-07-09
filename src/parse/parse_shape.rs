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

    let mut pattern: Option<Box<dyn Pattern>> = None;
    let mut diffuse: Option<f64> = None;
    let mut specular: Option<f64> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "pattern" => pattern = Some(parse_pattern(value)?),
            "diffuse" => diffuse = Some(parse_f64_from_integer_or_real(value)?),
            "specular" => specular = Some(parse_f64_from_integer_or_real(value)?),
            _ => todo!(),
        }
    }

    let mut material = Material::new();
    material.set_pattern(pattern.unwrap());
    material.set_diffuse(diffuse.unwrap());
    material.set_specular(specular.unwrap());

    Ok(material)
}

fn parse_pattern(value: &Yaml) -> Result<Box<dyn Pattern>, Box<dyn Error>> {
    let map = value.as_hash().unwrap();

    let mut pattern: Option<Box<dyn Pattern>> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "flat" => {
                let color = parse_color(value)?;
                pattern = Some(Box::new(FlatPattern::new(color)));
            }
            "stripes" => {
                pattern = Some(Box::new(parse_stripes(value)?));
            }
            "gradient" => {
                pattern = Some(Box::new(parse_gradient(value)?));
            }
            "checkers" => {
                pattern = Some(Box::new(parse_checkers(value)?));
            }
            "rings" => {
                pattern = Some(Box::new(parse_rings(value)?));
            }
            _ => todo!(),
        }
    }

    Ok(pattern.unwrap())
}

// TODO: Bunch of duplicated code between this and parse_gradient.
fn parse_stripes(value: &Yaml) -> Result<StripePattern, Box<dyn Error>> {
    let map = value.as_hash().unwrap();

    let mut background: Option<Color> = None;
    let mut stripe: Option<Color> = None;
    let mut transform: Option<Transform> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "colors" => {
                let value_vec = value.as_vec().unwrap().to_owned();

                background = Some(parse_color(value_vec.get(0).unwrap())?);
                stripe = Some(parse_color(value_vec.get(1).unwrap())?);
            }
            "transform" => {
                transform = Some(parse_transform(value)?);
            }
            _ => todo!(),
        }
    }

    let mut pattern = StripePattern::new(background.unwrap(), stripe.unwrap());

    if let Some(t) = transform {
        pattern.set_transform(t)
    }

    Ok(pattern)
}

fn parse_gradient(value: &Yaml) -> Result<GradientPattern, Box<dyn Error>> {
    let map = value.as_hash().unwrap();

    let mut starting_color: Option<Color> = None;
    let mut ending_color: Option<Color> = None;
    let mut transform: Option<Transform> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "colors" => {
                let value_vec = value.as_vec().unwrap().to_owned();

                starting_color = Some(parse_color(value_vec.get(0).unwrap())?);
                ending_color = Some(parse_color(value_vec.get(1).unwrap())?);
            }
            "transform" => {
                transform = Some(parse_transform(value)?);
            }
            _ => todo!(),
        }
    }

    let mut pattern = GradientPattern::new(starting_color.unwrap(), ending_color.unwrap());

    if let Some(t) = transform {
        pattern.set_transform(t)
    }

    Ok(pattern)
}

fn parse_checkers(value: &Yaml) -> Result<Checker3DPattern, Box<dyn Error>> {
    let map = value.as_hash().unwrap();

    let mut background: Option<Color> = None;
    let mut checker: Option<Color> = None;
    let mut transform: Option<Transform> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "colors" => {
                let value_vec = value.as_vec().unwrap().to_owned();

                background = Some(parse_color(value_vec.get(0).unwrap())?);
                checker = Some(parse_color(value_vec.get(1).unwrap())?);
            }
            "transform" => {
                transform = Some(parse_transform(value)?);
            }
            _ => todo!(),
        }
    }

    let mut pattern = Checker3DPattern::new(background.unwrap(), checker.unwrap());

    if let Some(t) = transform {
        pattern.set_transform(t)
    }

    Ok(pattern)
}

fn parse_rings(value: &Yaml) -> Result<RingPattern, Box<dyn Error>> {
    let map = value.as_hash().unwrap();

    let mut starting_color: Option<Color> = None;
    let mut ending_color: Option<Color> = None;
    let mut transform: Option<Transform> = None;

    for (key, value) in map {
        match key.as_str().unwrap() {
            "colors" => {
                let value_vec = value.as_vec().unwrap().to_owned();

                starting_color = Some(parse_color(value_vec.get(0).unwrap())?);
                ending_color = Some(parse_color(value_vec.get(1).unwrap())?);
            }
            "transform" => {
                transform = Some(parse_transform(value)?);
            }
            _ => todo!(),
        }
    }

    let mut pattern = RingPattern::new(starting_color.unwrap(), ending_color.unwrap());

    if let Some(t) = transform {
        pattern.set_transform(t)
    }

    Ok(pattern)
}

fn parse_transform(yaml: &Yaml) -> Result<Transform, Box<dyn Error>> {
    let nodes = yaml.as_vec().unwrap();

    let mut transform = Transform::new(IDENTITY_MATRIX);

    for node in nodes {
        let map = node.as_hash().unwrap();

        for (key, value) in map {
            match key.as_str().unwrap() {
                "translate" => {
                    let values = parse_values(value.as_vec().unwrap().to_owned()).unwrap();
                    let translation = Transform::translation(
                        values[0].unwrap(),
                        values[1].unwrap(),
                        values[2].unwrap(),
                    );
                    transform = transform * translation;
                }
                "scale" => {
                    let values = parse_values(value.as_vec().unwrap().to_owned()).unwrap();
                    let scaling = Transform::scaling(
                        values[0].unwrap(),
                        values[1].unwrap(),
                        values[2].unwrap(),
                    );
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
                    let mut shear_values = [0.0f64; 6];
                    let value_vec = value.as_vec().unwrap();

                    for (i, value) in value_vec.iter().enumerate() {
                        shear_values[i] = parse_f64_from_integer_or_real(value)?;
                    }

                    let shearing = Transform::shearing(
                        shear_values[0],
                        shear_values[1],
                        shear_values[2],
                        shear_values[3],
                        shear_values[4],
                        shear_values[5],
                    );

                    transform = transform * shearing;
                }
                _ => todo!(),
            }
        }
    }

    Ok(transform)
}
