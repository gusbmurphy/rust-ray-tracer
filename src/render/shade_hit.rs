use crate::prelude::*;

pub fn shade_hit(world: &World, hit: &Intersection<Sphere>) -> Color {
    let eye_vector = -hit.ray().direction().to_owned();

    let lighting_calculator = LightingCalculator::new(
        eye_vector,
        hit.normal_vector(),
        world.light().to_owned().unwrap(),
    );

    let adjusted_hit = adjust_hit(hit);
    let hit_is_in_shadow = world.is_point_shadowed(&adjusted_hit);

    return lighting_calculator.color_for_material_at(
        hit.object().material().to_owned(),
        adjusted_hit,
        hit_is_in_shadow,
    );
}

fn adjust_hit(hit: &Intersection<Sphere>) -> Point {
    hit.point() + adjust_normal_vector(hit) * EPSILON
}

fn adjust_normal_vector(hit: &Intersection<Sphere>) -> Vector {
    if hit.is_inside_object() {
        -hit.normal_vector()
    } else {
        hit.normal_vector()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shading_an_intersection() {
        let world = World::create_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let hit = ray.cast_into(&world).unwrap();

        let result = shade_hit(&world, &hit);

        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::create_default();

        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.25, 0.0));
        world.set_light(light);

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let hit = ray.cast_into(&world).unwrap();

        let result = shade_hit(&world, &hit);

        assert_eq!(result, Color::new(0.90498, 0.90498, 0.90498))
    }
}
