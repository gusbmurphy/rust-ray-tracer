use crate::{
    pattern::{FlatPattern, Pattern},
    prelude::*,
};

#[derive(Debug)]
pub struct Material {
    pattern: Box<dyn Pattern + Sync + Send>,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    refractive_index: f64,
    transparency: f64,
}

impl Material {
    pub fn color_at(&self, point: &Point) -> Color {
        self.pattern.color_at(point)
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> &f64 {
        &self.diffuse
    }

    pub fn specular(&self) -> &f64 {
        &self.specular
    }

    pub fn shininess(&self) -> &f64 {
        &self.shininess
    }

    pub fn reflective(&self) -> &f64 {
        &self.reflective
    }

    pub fn refractive_index(&self) -> &f64 {
        &self.refractive_index
    }

    pub fn transparency(&self) -> &f64 {
        &self.transparency
    }
}

pub struct MaterialBuilder {
    pattern: Box<dyn Pattern + Sync>,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    refractive_index: f64,
    transparency: f64,
}

impl MaterialBuilder {
    pub fn new() -> Self {
        MaterialBuilder {
            pattern: Box::new(FlatPattern::new(Color::new(1.0, 1.0, 1.0))),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }

    pub fn pattern(mut self, pattern: Box<dyn Pattern>) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn flat_color(mut self, color: Color) -> Self {
        self.pattern = Box::new(FlatPattern::new(color));
        self
    }

    pub fn ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn build(self) -> Material {
        Material {
            pattern: self.pattern,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
            reflective: self.reflective,
            refractive_index: self.refractive_index,
            transparency: self.transparency,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        return self.pattern.eq(&other.pattern)
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_material() {
        let default_material = MaterialBuilder::new().build();

        assert_eq!(default_material.ambient, 0.1);
        assert_eq!(default_material.diffuse, 0.9);
        assert_eq!(default_material.specular, 0.9);
        assert_eq!(default_material.shininess, 200.0);
        assert_eq!(default_material.reflective, 0.0);
        assert_eq!(default_material.refractive_index, 1.0);
        assert_eq!(default_material.transparency, 0.0);
    }
}
