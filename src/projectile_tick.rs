use crate::tuple::Tuple;

pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Environment { gravity, wind }
    }
}

#[derive(Copy, Clone)]
pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile { position, velocity }
    }

    pub fn get_position(&self) -> &Tuple {
        &self.position
    }
}

pub fn tick(e: &Environment, p: Projectile) -> Projectile {
    Projectile::new(p.position + p.velocity, p.velocity + e.gravity + e.wind)
}
