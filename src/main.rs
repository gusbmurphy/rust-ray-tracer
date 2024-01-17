pub use crate::projectile_tick::*;

mod point;
mod projectile_tick;
mod tuple;
mod vector;

mod prelude {
    pub use crate::point::*;
    pub use crate::tuple::*;
}

use prelude::*;
use vector::Vector;

fn main() {
    let mut projectile = Projectile::new(Point::new(0.0, 0.1, 0.0), Vector::new(1.0, 1.0, 1.0));

    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(0.0, -0.1, 0.0));

    while projectile.get_position().get_y() > 0.0 {
        let x = projectile.get_position().get_x();
        let y = projectile.get_position().get_y();
        let z = projectile.get_position().get_z();

        println!("Projectile is at {:?}", (x, y, z));
        projectile = tick(&environment, projectile);
    }
}
