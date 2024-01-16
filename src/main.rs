use projectile_tick::{Projectile, Environment, tick};
use tuple::Tuple;

mod projectile_tick;
mod tuple;

fn main() {
    let mut projectile = Projectile::new(
        Tuple::new_point(0.0, 0.1, 0.0),
        Tuple::new_vector(1.0, 1.0, 1.0),
    );

    let environment = Environment::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(0.0, -0.1, 0.0)
    );

    while projectile.get_position().get_y() > 0.0 {
        let x = projectile.get_position().get_x();
        let y = projectile.get_position().get_y();
        let z = projectile.get_position().get_z();

        println!("Projectile is at {:?}", (x, y, z));
        projectile = tick(&environment, projectile);
    }
}
