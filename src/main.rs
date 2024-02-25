use crate::color::Color;
pub use crate::projectile_tick::*;
use std::fs::File;
use std::io::prelude::*;

mod canvas;
mod color;
mod matrix;
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

const EPSILON: f32 = 0.00001;

fn main() -> std::io::Result<()> {
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile::new(Point::new(0.0, 1.0, 0.0), velocity);

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);

    let environment = Environment::new(gravity, wind);

    let canvas_height = 500;
    let mut canvas = Canvas::new(900, canvas_height);

    while projectile.get_position().get_y() > 0.0 {
        let x = projectile.get_position().get_x();
        let y = projectile.get_position().get_y();
        let z = projectile.get_position().get_z();

        canvas.write_pixel(
            x.round() as usize,
            (canvas_height as usize) - y.round() as usize,
            Color::new(1.0, 0.0, 0.0),
        );
        println!("Projectile is at {:?}", (x, y, z));
        projectile = tick(&environment, projectile);
    }

    let ppm_data = create_ppm_from_canvas(canvas);
    let mut file = File::create("projectile.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
