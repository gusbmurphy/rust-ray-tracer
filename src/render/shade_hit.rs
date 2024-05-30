use crate::prelude::*;
use crate::render::precomputation::Precomputation;

pub fn shade_hit(world: &World, precomputation: &Precomputation<Sphere>) -> Color {
    let lighting_calculator = LightingCalculator::new(
        precomputation.get_eye_vector(),
        precomputation.get_normal_vector(),
        world.get_light().to_owned().unwrap(),
    );

    return lighting_calculator.get_color_for_material_at(
        *precomputation.get_object().get_material(), precomputation.get_hit_point()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shading_an_intersection() {
        let world = World::get_default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let hit_object = world.get_object(0).unwrap();
        let intersection = Intersection::new(4.0, hit_object);

        let precomputation = Precomputation::new(&intersection, &ray);

        let result = shade_hit(&world, &precomputation);

        assert_eq!(result, Color::new(0.38066, 0.47583, 0.2855))
    }
}
