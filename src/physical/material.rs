use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_ambient(&self) -> f32 {
        self.ambient
    }

    pub fn get_diffuse(&self) -> f32 {
        self.diffuse
    }

    pub fn get_specular(&self) -> f32 {
        self.specular
    }

    pub fn get_shininess(&self) -> f32 {
        self.shininess
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_material() {
        let default_material = Material::new();

        assert_eq!(default_material.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(default_material.ambient, 0.1);
        assert_eq!(default_material.diffuse, 0.9);
        assert_eq!(default_material.specular, 0.9);
        assert_eq!(default_material.shininess, 200.0);
    }
}