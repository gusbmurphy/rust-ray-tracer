use crate::prelude::*;

// TODO: Would be nice to have a `ShapeBuilder` that could make any shape...
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

impl PartialEq for dyn Shape + Sync + Send {
    fn eq(&self, other: &(dyn Shape + Sync + Send)) -> bool {
        self.shape_type() == other.shape_type() && self.transform() == other.transform()
    }
}

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
    Plane,
}
