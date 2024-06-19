use crate::prelude::*;

pub struct World {
    light: Option<PointLight>, // I think this needs to be non-optional
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

    pub fn get_intersections_for<'a, 'b>(&'a self, ray: &'b Ray) -> Vec<Intersection<Sphere>>
    where
        'b: 'a,
    {
        let mut intersections = Vec::new();

        for object in self.objects.as_slice() {
            if let Some(actual_intersections) = ray.intersections_with(&object) {
                intersections.extend(Vec::from(actual_intersections));
            }
        }

        intersections.sort_by(|a, b| a.get_t().total_cmp(&b.get_t()));

        intersections
    }

    pub fn get_object(&self, index: usize) -> Option<&Sphere> {
        self.objects.get(index)
    }

    pub fn get_light(&self) -> &Option<PointLight> {
        &self.light
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn color_for_ray(&self, ray: Ray) -> Color {
        let intersections = self.get_intersections_for(&ray);
        let hit_intersection = determine_hit(intersections);

        if let Some(hit) = hit_intersection {
            let precomputation = Precomputation::new(&hit, &ray);
            return shade_hit(self, &precomputation);
        }

        return BLACK;
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.objects.push(sphere);
    }

    pub fn is_point_shadowed(&self, point: &Point) -> bool {
        let point_to_light_vector = self.light.unwrap().get_position() - point.to_owned();
        let point_to_light_ray = Ray::new(point.to_owned(), point_to_light_vector.normalize());

        let intersections = self.get_intersections_for(&point_to_light_ray);

        let possible_hit = determine_hit(intersections);

        if let Some(hit) = possible_hit {
            let distance_from_point_to_light = point_to_light_vector.get_magnitude();
            if *hit.get_t() < distance_from_point_to_light {
                return true;
            }
        }

        return false;
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

    #[test]
    fn intersecting_a_ray_with_a_world_gets_all_hits_sorted_in_ascending_order() {
        let world = World::get_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let intersections: Vec<Intersection<Sphere>> = world.get_intersections_for(&ray);

        assert_eq!(intersections.len(), 4);

        assert_eq!(*intersections[0].get_t(), 4.0);
        assert_eq!(*intersections[1].get_t(), 4.5);
        assert_eq!(*intersections[2].get_t(), 5.5);
        assert_eq!(*intersections[3].get_t(), 6.0);
    }

    #[test]
    fn getting_an_object_returns_the_first_one() {
        let world = World::get_default();

        let found_object = world.get_object(0).unwrap();

        let mut expected_material = Material::new();
        expected_material.set_color(Color::new(0.8, 1.0, 0.6));
        expected_material.set_specular(0.2);
        expected_material.set_diffuse(0.7);

        let mut expected_object = Sphere::new();
        expected_object.set_material(expected_material);

        assert_eq!(found_object.to_owned(), expected_object);
    }

    #[test]
    fn getting_an_object_returns_nothing_for_a_wild_index() {
        let world = World::get_default();

        let result = world.get_object(4234);

        assert_eq!(result, None);
    }

    #[test]
    fn color_when_ray_misses_everything() {
        let world = World::get_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

        let result = world.color_for_ray(ray);

        assert_eq!(result, BLACK);
    }

    #[test]
    fn color_for_a_ray_that_hits() {
        let world = World::get_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let result = world.color_for_ray(ray);

        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn this_hit_in_the_default_should_not_be_in_a_shadow_when_adjusted() {
        let world = World::get_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        // The intersection will happen with the outermost sphere...
        let intersection = Intersection::new(4.0, world.get_object(0).unwrap());
        let computation = Precomputation::new(&intersection, &ray);

        let is_point_shadowed = world.is_point_shadowed(&computation.get_adjusted_hit_point());
        assert_eq!(is_point_shadowed, false);
    }

    #[test]
    fn color_for_a_ray_that_hits_but_originates_inside_a_different_object() {
        let mut world = World::get_default();
        let spheres = world.objects.as_slice();

        let mut modified_spheres: Vec<Sphere> = Vec::new();

        // Setting the ambient value of each sphere's material to 1...
        for sphere in spheres {
            let mut material = sphere.get_material().to_owned();
            material.set_ambient(1.0);

            let mut modified_sphere = sphere.clone();
            modified_sphere.set_material(material);

            modified_spheres.push(modified_sphere);
        }

        world.objects = modified_spheres;

        // This ray originates inside of the outermost sphere, and is pointed at the inner one.
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

        let result = world.color_for_ray(ray);

        // Since the ambient is 1, the color will just be the color of that inner sphere.
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = World::get_default();
        let point = Point::new(0.0, 10.0, 0.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, false);
    }

    #[test]
    fn a_point_on_the_opposite_side_of_an_object_to_a_sphere_is_shadowed() {
        let world = World::get_default();
        let point = Point::new(10.0, -10.0, 10.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, true);
    }

    #[test]
    fn when_the_light_is_between_the_object_and_point_there_is_no_shadow() {
        let world = World::get_default();
        let point = Point::new(-20.0, 20.0, -20.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, false);
    }

    #[test]
    fn when_the_point_is_between_the_object_and_light_there_is_no_shadow() {
        let world = World::get_default();
        let point = Point::new(-2.0, 2.0, -2.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, false);
    }

    #[test]
    fn getting_the_color_for_a_shaded_hit() {
        let mut world = World::new();
        world.set_light(PointLight::new(
            Color::new(1.0, 1.0, 1.0),
            Point::new(0.0, 0.0, -10.0),
        ));

        let sphere_one = Sphere::new();
        world.add_sphere(sphere_one);

        let mut sphere_two = Sphere::new();
        sphere_two.set_transform(Transform::new_translation(0.0, 0.0, -5.0));
        world.add_sphere(sphere_two);

        let ray = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0));

        let result = world.color_for_ray(ray);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
