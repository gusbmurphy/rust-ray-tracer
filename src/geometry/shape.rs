use crate::prelude::*;

pub trait Shape: Sized {
    fn normal_at(&self, world_space_point: Point) -> Vector;
    fn intersections_with<'s, 'r>(&'s self, ray: &'r Ray) -> Vec<Intersection<Self>>
    where
        'r: 's;
    fn transform(&self) -> &Transform;
    fn set_transform(&mut self, transformation: Transform);
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
}
