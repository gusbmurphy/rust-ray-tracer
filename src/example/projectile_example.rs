use crate::example::projectile_tick::*;
use crate::render::canvas::*;
use crate::color::*;
use crate::point::*;
use crate::render::ppm::*;
use crate::tuple::*;
use crate::vector::*;
use std::fs::File;
use std::io::prelude::*;

pub fn draw_projectile_example_to_file() -> std::io::Result<()> {
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

        canvas.write_pixel(
            x.round() as usize,
            (canvas_height as usize) - y.round() as usize,
            Color::new(1.0, 0.0, 0.0),
        );
        projectile = tick(&environment, projectile);
    }

    let ppm_data = create_ppm_from_canvas(canvas);
    let mut file = File::create("projectile.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
