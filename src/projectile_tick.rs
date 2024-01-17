use crate::{point::Point, vector::Vector};

pub struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Environment { gravity, wind }
    }
}

#[derive(Copy, Clone)]
pub struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Projectile { position, velocity }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }
}

pub fn tick(e: &Environment, p: Projectile) -> Projectile {
    Projectile::new(p.position + p.velocity, p.velocity + e.gravity + e.wind)
}
