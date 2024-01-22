use crate::color::Color;
pub use crate::projectile_tick::*;
use std::fs::File;
use std::io::prelude::*;

mod canvas;
mod color;
mod point;
mod ppm;
mod projectile_tick;
mod tuple;
mod vector;

mod prelude {
    pub use crate::point::*;
    pub use crate::tuple::*;
}

use canvas::Canvas;
use ppm::create_ppm_from_canvas;
use prelude::*;
use vector::Vector;

fn main() -> std::io::Result<()> {
    let mut projectile = Projectile::new(Point::new(0.0, 0.1, 0.0), Vector::new(1.0, 1.0, 1.0));

    let environment = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(0.0, -0.1, 0.0));

    let mut canvas = Canvas::new(15, 15);

    while projectile.get_position().get_y() > 0.0 {
        let x = projectile.get_position().get_x();
        let y = projectile.get_position().get_y();
        let z = projectile.get_position().get_z();

        canvas.write_pixel(x.round() as usize, y.round() as usize, Color::new(1.0, 0.0, 0.0));
        println!("Projectile is at {:?}", (x, y, z));
        projectile = tick(&environment, projectile);
    }

    let ppm_data = create_ppm_from_canvas(canvas);
    let mut file = File::create("projectile.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
