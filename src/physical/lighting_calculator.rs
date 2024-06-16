use crate::physical::point_light::PointLight;
use crate::prelude::*;

pub struct LightingCalculator {
    eye_vector: Vector,
    normal_vector: Vector,
    light: PointLight,
}

impl LightingCalculator {
    pub fn new(eye_vector: Vector, normal_vector: Vector, light: PointLight) -> Self {
        LightingCalculator {
            eye_vector,
            normal_vector,
            light,
        }
    }

    pub fn get_color_for_material_at(&self, material: Material, position: Point, in_shadow: bool) -> Color {
        let effective_color = material.get_color() * self.light.get_intensity();

        let light_vector = (self.light.get_position() - position).normalize();

        let light_dot_normal = dot(&light_vector, &self.normal_vector);

        let diffuse_contribution: Color;
        let specular_contribution: Color;

        if light_dot_normal < 0.0 {
            // This means the light is opposite the normal vector...
            diffuse_contribution = BLACK;
            specular_contribution = BLACK;
        } else {
            diffuse_contribution = effective_color * material.get_diffuse() * light_dot_normal;

            let reflection_vector = (-light_vector).reflect_around(&self.normal_vector);
            let reflection_dot_eye = dot(&reflection_vector, &self.eye_vector);

            if reflection_dot_eye < 0.0 {
                // This means the light reflects away from the eye...
                specular_contribution = BLACK;
            } else {
                let specular_factor = reflection_dot_eye.powf(material.get_shininess());
                specular_contribution =
                    self.light.get_intensity() * material.get_specular() * specular_factor;
            }
        }

        let ambient_contribution = effective_color * material.get_ambient();

        if in_shadow {
            return ambient_contribution;
        }

        return ambient_contribution + diffuse_contribution + specular_contribution;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eye_between_light_and_point() {
        let material = Material::new();
        let point = Point::new(0.0, 0.0, 0.0);

        let calculator = LightingCalculator {
            eye_vector: Vector::new(0.0, 0.0, -1.0),
            normal_vector: Vector::new(0.0, 0.0, -1.0),
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0)),
        };

        let result = calculator.get_color_for_material_at(material, point, false);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn eye_between_light_and_point_and_eye_offset_45_degrees() {
        let material = Material::new();
        let point = Point::new(0.0, 0.0, 0.0);

        let calculator = LightingCalculator {
            eye_vector: Vector::new(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0),
            normal_vector: Vector::new(0.0, 0.0, -1.0),
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0)),
        };

        let result = calculator.get_color_for_material_at(material, point, false);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn eye_between_light_and_point_and_light_offset_45_degrees() {
        let material = Material::new();
        let point = Point::new(0.0, 0.0, 0.0);

        let calculator = LightingCalculator {
            eye_vector: Vector::new(0.0, 0.0, -1.0),
            normal_vector: Vector::new(0.0, 0.0, -1.0),
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0)),
        };

        let result = calculator.get_color_for_material_at(material, point, false);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn eye_in_path_of_reflection_vector() {
        let material = Material::new();
        let point = Point::new(0.0, 0.0, 0.0);

        let calculator = LightingCalculator {
            eye_vector: Vector::new(0.0, -2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0),
            normal_vector: Vector::new(0.0, 0.0, -1.0),
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0)),
        };

        let result = calculator.get_color_for_material_at(material, point, false);

        assert_eq!(result, Color::new(1.63638, 1.63638, 1.63638));
    }

    #[test]
    fn light_on_opposite_side_of_surface() {
        let material = Material::new();
        let point = Point::new(0.0, 0.0, 0.0);

        let calculator = LightingCalculator {
            eye_vector: Vector::new(0.0, 0.0, -1.0),
            normal_vector: Vector::new(0.0, 0.0, -1.0),
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0)),
        };

        let result = calculator.get_color_for_material_at(material, point, false);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn when_the_surface_is_in_a_shadow_we_only_use_the_ambient_component() {
        let material = Material::new();
        let point = Point::new(0.0, 0.0, 0.0);

        let calculator = LightingCalculator {
            eye_vector: Vector::new(0.0, 0.0, -1.0),
            normal_vector: Vector::new(0.0, 0.0, -1.0),
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0)),
        };

        let result = calculator.get_color_for_material_at(material, point, true);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
