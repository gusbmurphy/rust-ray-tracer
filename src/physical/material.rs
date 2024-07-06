use crate::{
    pattern::{self, FlatPattern, Pattern},
    prelude::*,
};

#[derive(Debug)]
pub struct Material {
    pattern: Box<dyn Pattern>,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

// TODO: What if there was a builder for this sort of thing? Instead of having a million setter
// methods...
impl Material {
    pub fn new() -> Self {
        let pattern = Box::new(FlatPattern::new(Color::new(1.0, 1.0, 1.0)));

        Material {
            pattern,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn color_at(&self, point: &Point) -> Color {
        self.pattern.color_at(point)
    }

    pub fn set_flat_color(&mut self, color: Color) {
        self.pattern = Box::new(FlatPattern::new(color));
    }

    pub fn set_pattern(&mut self, pattern: Box<dyn Pattern>) {
        self.pattern = pattern;
    }

    pub fn ambient(&self) -> f32 {
        self.ambient
    }

    pub fn set_ambient(&mut self, ambient: f32) {
        self.ambient = ambient;
    }

    pub fn diffuse(&self) -> &f32 {
        &self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: f32) {
        self.diffuse = diffuse;
    }

    pub fn specular(&self) -> &f32 {
        &self.specular
    }

    pub fn set_specular(&mut self, specular: f32) {
        self.specular = specular;
    }

    pub fn shininess(&self) -> &f32 {
        &self.shininess
    }

    pub fn set_shininess(&mut self, shininess: f32) {
        self.shininess = shininess;
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
        let default_material = Material::new();

        assert_eq!(default_material.ambient, 0.1);
        assert_eq!(default_material.diffuse, 0.9);
        assert_eq!(default_material.specular, 0.9);
        assert_eq!(default_material.shininess, 200.0);
    }
}
