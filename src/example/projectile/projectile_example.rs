use crate::example::projectile::tick;
use crate::example::projectile::Environment;
use crate::example::projectile::Projectile;
use crate::prelude::*;
use crate::render::canvas::Canvas;
use crate::render::color::*;
use crate::render::ppm::*;

pub fn draw_projectile_example_ppm() -> String {
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile::new(Point::new(0.0, 1.0, 0.0), velocity);

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);

    let environment = Environment::new(gravity, wind);

    let canvas_height = 500;
    let mut canvas = Canvas::new(900, canvas_height);

    while *projectile.get_position().y() > 0.0 {
        let x = projectile.get_position().x();
        let y = projectile.get_position().y();

        canvas.write_pixel(
            x.round() as usize,
            (canvas_height as usize) - y.round() as usize,
            Color::new(1.0, 0.0, 0.0),
        );
        projectile = tick(&environment, projectile);
    }

    create_ppm_from_canvas(canvas)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snapshot() {
        let result = draw_projectile_example_ppm();
        insta::assert_yaml_snapshot!(result);
    }
}
