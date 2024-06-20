use crate::prelude::*;
use crate::render::precomputation::Precomputation;

pub fn shade_hit(world: &World, precomputation: &Precomputation<Sphere>) -> Color {
    let lighting_calculator = LightingCalculator::new(
        precomputation.eye_vector(),
        precomputation.normal_vector(),
        world.light().to_owned().unwrap(),
    );

    let adjusted_hit = precomputation.adjusted_hit_point();
    let hit_is_in_shadow = world.is_point_shadowed(&adjusted_hit);

    return lighting_calculator.color_for_material_at(
        *precomputation.intersected_object().material(),
        adjusted_hit,
        hit_is_in_shadow,
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shading_an_intersection() {
        let world = World::create_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let hit = ray.cast_into(&world).unwrap();

        let precomputation = Precomputation::new(&hit);

        let result = shade_hit(&world, &precomputation);

        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::create_default();

        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.25, 0.0));
        world.set_light(light);

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

        let hit = ray.cast_into(&world).unwrap();

        let precomputation = Precomputation::new(&hit);

        let result = shade_hit(&world, &precomputation);

        assert_eq!(result, Color::new(0.90498, 0.90498, 0.90498))
    }
}
