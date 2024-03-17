use crate::canvas::*;
use crate::color::*;
use crate::point::*;
use crate::ppm::*;
use crate::transformation::*;
use crate::tuple::*;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

pub fn draw_clock_example(canvas_size: u64, clock_radius: u64) -> std::io::Result<()> {
    let mut canvas = Canvas::new(canvas_size, canvas_size);

    let half_size = canvas_size / 2;
    let center = Point::new(half_size as f32, half_size as f32, 0.0);

    canvas.write_pixel(
        center.get_x().round() as usize,
        center.get_y().round() as usize,
        Color::new(1.0, 0.0, 0.0),
    );

    let translation = Transformation::new_translation(0.0, clock_radius as f32, 0.0);

    let twelve_o_clock = translation * center;
    let mut point_to_draw = twelve_o_clock;

    let twelth_rotation = Transformation::new_y_rotation(30.0 * PI / 180.0);

    for _i in 1..12 {
        canvas.write_pixel(
            point_to_draw.get_x().round() as usize,
            point_to_draw.get_y().round() as usize,
            Color::new(1.0, 1.0, 1.0),
        );

        point_to_draw = twelth_rotation * point_to_draw;
    }

    let ppm_data = create_ppm_from_canvas(canvas);
    let mut file = File::create("clock.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
