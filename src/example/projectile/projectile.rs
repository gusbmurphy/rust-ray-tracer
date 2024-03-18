use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Projectile { position, velocity }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }
}
