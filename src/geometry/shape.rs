use crate::prelude::*;

pub trait Shape {
    fn normal_at(&self, world_space_point: Point) -> Vector;
}
