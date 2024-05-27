use crate::prelude::*;

pub struct World {
    light: Option<PointLight>,
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            light: None,
            objects: Vec::new(),
        }
    }

    pub fn get_default() -> Self {
        let mut first_sphere_material = Material::new();
        first_sphere_material.set_color(Color::new(0.8, 1.0, 0.6));
        first_sphere_material.set_specular(0.2);
        first_sphere_material.set_diffuse(0.7);

        let mut first_sphere = Sphere::new();
        first_sphere.set_material(first_sphere_material);

        let second_sphere_scaling = Transform::new_scaling(0.5, 0.5, 0.5);
        let mut second_sphere = Sphere::new();
        second_sphere.set_transform(second_sphere_scaling);

        World {
            light: Some(PointLight::new(
                Color::new(1.0, 1.0, 1.0),
                Point::new(-10.0, 10.0, -10.0),
            )),
            objects: Vec::from([first_sphere, second_sphere]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_world_is_empty() {
        let world = World::new();

        assert!(world.light.is_none());
        assert!(world.objects.is_empty());
    }

    #[test]
    fn default_world_has_expected_contents() {
        let default_world = World::get_default();

        let default_light = default_world.light.unwrap();
        assert_eq!(default_light.get_intensity(), Color::new(1.0, 1.0, 1.0));
        assert_eq!(default_light.get_position(), Point::new(-10.0, 10.0, -10.0));

        let default_spheres = default_world.objects;
        assert_eq!(default_spheres.len(), 2);

        assert!(default_spheres.iter().any(|sphere| {
            let material = sphere.get_material();

            material.get_color() == Color::new(0.8, 1.0, 0.6)
                && material.get_diffuse() == 0.7
                && material.get_specular() == 0.2
        }));

        assert!(default_spheres.iter().any(|sphere| {
            sphere.get_transform().to_owned() == Transform::new_scaling(0.5, 0.5, 0.5)
        }));
    }
}
