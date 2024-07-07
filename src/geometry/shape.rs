use crate::prelude::*;

pub trait Shape {
    fn normal_at(&self, world_space_point: Point) -> Vector;
    fn times_of_intersections_with<'s, 'r>(&'s self, ray: &'r Ray) -> Vec<f64>
    where
        'r: 's;
    fn transform(&self) -> &Transform;
    fn set_transform(&mut self, transformation: Transform);
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
    fn shape_type(&self) -> ShapeType;
}

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
    Plane,
}
