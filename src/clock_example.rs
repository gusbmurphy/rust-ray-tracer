use crate::canvas::*;
use crate::color::*;
use crate::point::*;
use crate::ppm::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::vector::Vector;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

pub fn draw_clock_example_to_file(canvas_size: u64, clock_radius: u64) -> std::io::Result<()> {
    let twelve_o_clock = Point::new(0.0, clock_radius as f32, 0.0);

    let mut points = Vec::new();
    points.push(twelve_o_clock);

    let twelth_rotation = Transformation::new_z_rotation(PI / 6.0);

    for i in 1..12 {
        let last_point = points[i - 1];
        let new_point = twelth_rotation * last_point;
        points.push(new_point);
    }

    let half_canvas_size = canvas_size as f32 / 2.0;
    let clock_center = Vector::new(half_canvas_size, half_canvas_size, 0.0);

    let mut translated_points = Vec::new();

    for point in points {
        let translated_point = point + clock_center;
        translated_points.push(translated_point);
    }

    let mut canvas = Canvas::new(canvas_size, canvas_size);

    for point in translated_points {
        canvas.write_pixel(
            point.get_x().round() as usize,
            point.get_y().round() as usize,
            Color::new(1.0, 1.0, 1.0),
        );
    }

    let ppm_data = create_ppm_from_canvas(canvas);
    let mut file = File::create("clock.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
