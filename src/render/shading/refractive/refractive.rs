use crate::prelude::*;

pub fn calculate_refractive_contribution(
    hit: &Intersection,
    world: &World,
    current_recursion_count: i8,
) -> Color {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn an_opaque_object_has_no_refractive_contribution() {
        let world = World::create_default();
        let object = world.shapes().get(0).unwrap();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersection::of(object, &ray);

        let result = calculate_refractive_contribution(intersections.get(0).unwrap(), &world, 0);

        assert_eq!(result, BLACK);
    }

    #[test]
    // Wow, this test is not nice to look at...
    fn an_actually_refracted_ray() {
        let mut ambient_sphere = Sphere::new();
        ambient_sphere.set_material(
            MaterialBuilder::new()
                .specular(0.2)
                .diffuse(0.7)
                .ambient(1.0) // Setting ambient to 1 so the returned color is simple...
                .pattern(Box::new(TestPattern::new()))
                .build(),
        );

        let scaling = Transform::scaling(0.5, 0.5, 0.5);
        let mut transparent_sphere = Sphere::new();
        transparent_sphere.set_transform(scaling);

        transparent_sphere.set_material(
            MaterialBuilder::new()
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        );

        let transparent_sphere_rc = Rc::new(transparent_sphere) as Rc<dyn Shape>;

        let mut world = World::new();
        world.add_shape(Rc::new(ambient_sphere));
        world.add_shape(transparent_sphere_rc.clone());

        let ray = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let intersections = Intersection::of(&transparent_sphere_rc, &ray);

        let result = calculate_refractive_contribution(intersections.get(0).unwrap(), &world, 0);

        assert_eq!(result, Color::new(0.0, 0.99888, 0.04726))
    }
}
