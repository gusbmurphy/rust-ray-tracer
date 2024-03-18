mod environment;
mod projectile;
mod projectile_example;

use environment::Environment;
use projectile::Projectile;
pub use projectile_example::draw_projectile_example_to_file;

pub fn tick(e: &Environment, p: Projectile) -> Projectile {
    Projectile::new(p.position + p.velocity, p.velocity + e.gravity + e.wind)
}
