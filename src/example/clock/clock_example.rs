use crate::prelude::*;
use crate::render::canvas::Canvas;
use crate::render::color::Color;
use crate::render::ppm::create_ppm_from_canvas;
use std::f32::consts::PI;

pub fn draw_clock_example_ppm(canvas_size: u64, clock_radius: u64) -> String {
    let twelve_o_clock = Point::new(0.0, clock_radius as f32, 0.0);

    let mut points = Vec::new();
    points.push(twelve_o_clock);

    let twelth_rotation = Transform::new_z_rotation(PI / 6.0);

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
            point.x().round() as usize,
            point.y().round() as usize,
            Color::new(1.0, 1.0, 1.0),
        );
    }

    create_ppm_from_canvas(canvas)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snapshot() {
        let result = draw_clock_example_ppm(100, 25);
        insta::assert_yaml_snapshot!(result);
    }
}
