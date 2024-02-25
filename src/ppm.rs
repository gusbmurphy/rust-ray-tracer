use crate::canvas::Canvas;
use crate::color::Color;

const MAX_PPM_COLOR_VALUE: u8 = 255;
const MAX_PPM_LINE_LENGTH: u8 = 70;

pub fn create_ppm_from_canvas(canvas: Canvas) -> String {
    let mut header = "P3\n".to_owned();
    header.push_str(format!("{} {}\n", canvas.get_width(), canvas.get_height()).as_str());
    header.push_str(format!("{}\n", MAX_PPM_COLOR_VALUE).as_str());

    let mut pixel_data = String::new();

    for row in canvas.get_rows() {
        let mut row_string = String::new();

        for color in row {
            let ppm_values = convert_color_to_ppm_values(color);

            for ppm_value in ppm_values {
                let value_string = ppm_value.to_string();

                if row_length_will_be_under_max_after_adding_space_and_value(
                    &row_string,
                    &value_string,
                ) {
                    if !row_string.is_empty() {
                        row_string.push(' ');
                    }
                    row_string.push_str(&value_string);
                } else {
                    row_string.push('\n');
                    pixel_data.push_str(&row_string);
                    row_string = String::from(value_string);
                }
            }
        }

        pixel_data.push_str(&row_string);
        pixel_data.push('\n');
    }

    String::from(header + pixel_data.as_str())
}

fn row_length_will_be_under_max_after_adding_space_and_value(row: &String, value: &String) -> bool {
    row.chars().count() + 1 + value.chars().count() < MAX_PPM_LINE_LENGTH as usize
}

fn convert_color_to_ppm_values(color: &Color) -> [u8; 3] {
    let r = convert_color_value_to_ppm_value(color.get_r());
    let b = convert_color_value_to_ppm_value(color.get_b());
    let g = convert_color_value_to_ppm_value(color.get_g());

    [r, b, g]
}

fn convert_color_value_to_ppm_value(value: f64) -> u8 {
    let ppm_value = ((MAX_PPM_COLOR_VALUE as f64) * value) as f64;

    if ppm_value < 0.0 {
        return 0;
    }

    if ppm_value > MAX_PPM_COLOR_VALUE as f64 {
        return MAX_PPM_COLOR_VALUE;
    }

    ppm_value.round() as u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructing_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = create_ppm_from_canvas(canvas);

        let expected_header = "\
            P3\n\
            5 3\n\
            255\n\
        ";

        assert!(ppm.starts_with(expected_header));
    }

    #[test]
    fn ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);

        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let ppm = create_ppm_from_canvas(canvas);

        let mut pixel_data = String::new();

        for (i, line) in ppm.lines().enumerate() {
            if i > 2 && i < 6 {
                pixel_data.push_str(line);
                pixel_data.push('\n');
            }
        }

        let expected_pixel_data = "\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n\
        ";

        assert_eq!(pixel_data, expected_pixel_data);
    }

    #[test]
    fn splitting_long_ppm_lines() {
        let mut canvas = Canvas::new(10, 2);

        for y in 0..2 {
            for x in 0..10 {
                canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }

        let ppm = create_ppm_from_canvas(canvas);

        let mut pixel_data = String::new();

        for (i, line) in ppm.lines().enumerate() {
            if i > 2 && i < 7 {
                pixel_data.push_str(line);
                pixel_data.push('\n');
            }
        }

        // No line can be more than 70 characters long...
        let expected_pixel_data = "\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
            153 255 204 153 255 204 153 255 204 153 255 204 153\n\
        ";

        assert_eq!(pixel_data, expected_pixel_data);
    }
}
