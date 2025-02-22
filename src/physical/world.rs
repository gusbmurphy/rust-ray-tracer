use crate::prelude::*;
use std::rc::Rc;

pub struct World {
    light: PointLight,
    shapes: Vec<Rc<dyn Shape>>,
    background: Color,
}

impl World {
    pub fn new() -> Self {
        World {
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)),
            shapes: Vec::new(),
            background: BLACK,
        }
    }

    // TODO: I think we can use the `Default` trait maybe...
    pub fn create_default() -> Self {
        let first_sphere_material = MaterialBuilder::new()
            .flat_color(Color::new(0.8, 1.0, 0.6))
            .specular(0.2)
            .diffuse(0.7)
            .build();

        let mut first_sphere = Sphere::new();
        first_sphere.set_material(first_sphere_material);

        let second_sphere_scaling = Transform::scaling(0.5, 0.5, 0.5);
        let mut second_sphere = Sphere::new();
        second_sphere.set_transform(second_sphere_scaling);

        let mut shapes: Vec<Rc<dyn Shape>> = Vec::new();
        shapes.push(Rc::new(first_sphere));
        shapes.push(Rc::new(second_sphere));

        World {
            light: PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0)),
            shapes,
            background: BLACK,
        }
    }

    pub fn intersections_for<'a, 'b>(&'a self, ray: &'b Ray) -> Vec<Intersection>
    where
        'b: 'a,
    {
        let mut intersections: Vec<Intersection<'_>> = Vec::new();

        for object in &self.shapes {
            let mut these_intersections = Intersection::of(object, ray);
            intersections.append(&mut these_intersections);
        }

        intersections.sort_by(|a, b| a.t().total_cmp(&b.t()));

        intersections
    }

    pub fn hit_for<'a, 'b>(&'a self, ray: &'b Ray) -> Option<Intersection>
    where
        'b: 'a,
    {
        let intersections = self.intersections_for(ray);
        determine_hit(intersections)
    }

    pub fn light(&self) -> &PointLight {
        &self.light
    }

    pub fn set_light(&mut self, light: PointLight) {
        self.light = light;
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.shapes.push(Rc::new(sphere));
    }

    pub fn add_plane(&mut self, plane: Plane) {
        self.shapes.push(Rc::new(plane));
    }

    pub fn is_point_shadowed(&self, point: &Point) -> bool {
        let point_to_light_vector = *self.light.position() - point.to_owned();
        let point_to_light_ray = Ray::new(point.to_owned(), point_to_light_vector.normalize());

        let possible_hit = self.hit_for(&point_to_light_ray);

        if let Some(hit) = possible_hit {
            let distance_from_point_to_light = point_to_light_vector.magnitude();
            if *hit.t() < distance_from_point_to_light {
                return true;
            }
        }

        return false;
    }

    pub fn shapes(&self) -> &Vec<Rc<dyn Shape>> {
        &self.shapes
    }

    pub fn set_shapes(&mut self, shapes: Vec<Rc<dyn Shape>>) {
        self.shapes = shapes;
    }

    pub fn add_shape(&mut self, shape: Rc<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }

    pub fn background(&self) -> &Color {
        &self.background
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersecting_a_ray_with_a_world_gets_all_hits_sorted_in_ascending_order() {
        let world = World::create_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let intersections: Vec<Intersection> = world.intersections_for(&ray);

        assert_eq!(intersections.len(), 4);

        assert_eq!(*intersections[0].t(), 4.0);
        assert_eq!(*intersections[1].t(), 4.5);
        assert_eq!(*intersections[2].t(), 5.5);
        assert_eq!(*intersections[3].t(), 6.0);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = World::create_default();
        let point = Point::new(0.0, 10.0, 0.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, false);
    }

    #[test]
    fn a_point_on_the_opposite_side_of_an_object_to_a_sphere_is_shadowed() {
        let world = World::create_default();
        let point = Point::new(10.0, -10.0, 10.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, true);
    }

    #[test]
    fn when_the_light_is_between_the_object_and_point_there_is_no_shadow() {
        let world = World::create_default();
        let point = Point::new(-20.0, 20.0, -20.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, false);
    }

    #[test]
    fn when_the_point_is_between_the_object_and_light_there_is_no_shadow() {
        let world = World::create_default();
        let point = Point::new(-2.0, 2.0, -2.0);
        let result = world.is_point_shadowed(&point);
        assert_eq!(result, false);
    }
}
