use core::slice;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    point: Point,
    normal: Vector,
    transform: Transform,
    material: Material,
}

impl Plane {
    pub fn new(point: Point, normal: Vector) -> Self {
        Plane {
            point,
            normal,
            transform: Transform::new(IDENTITY_MATRIX),
            material: Material::new(),
        }
    }
}

impl Shape for Plane {
    fn normal_at(&self, world_space_point: Point) -> Vector {
        self.normal.clone()
    }

    fn intersections_with<'s, 'r>(&'s self, ray: &'r Ray) -> Vec<Intersection<Self>>
    where
        'r: 's,
    {
        todo!()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transformation: Transform) {
        self.transform = transformation;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_normal_vector_is_always_the_same() {
        let plane = Plane::new(ORIGIN, POSITIVE_Y);

        assert_eq!(plane.normal_at(ORIGIN), POSITIVE_Y);
        assert_eq!(plane.normal_at(Point::new(1.0, 0.0, 0.0)), POSITIVE_Y);
        assert_eq!(plane.normal_at(Point::new(8.0, 0.0, -3.0)), POSITIVE_Y);
    }
}
