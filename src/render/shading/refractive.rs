use crate::prelude::*;

pub fn calculate_refractive_contribution(hit: &Intersection) -> Color {
    BLACK
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn an_opaque_object_has_no_refractive_contribution() {
        let world = World::create_default();
        let object = world.shapes().get(0).unwrap();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersection::of(object, &ray);

        let result = calculate_refractive_contribution(intersections.get(0).unwrap());

        assert_eq!(result, BLACK);
    }
}
