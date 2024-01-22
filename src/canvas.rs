use crate::color::Color;
use std::cmp;

pub struct Canvas {
    width: u8,
    height: u8,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u8, height: u8) -> Self {
        Canvas {
            width,
            height,
            pixels: create_all_default_rows(width, height),
        }
    }

    pub fn get_width(&self) -> u8 {
        return self.width;
    }

    pub fn get_height(&self) -> u8 {
        return self.height;
    }

    pub fn get_rows(&self) -> &Vec<Vec<Color>> {
        return &self.pixels;
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn get_pixel_at(&self, x: usize, y: usize) -> Color {
        return self.pixels[y][x];
    }
}

fn create_all_default_rows(width: u8, height: u8) -> Vec<Vec<Color>> {
    let mut rows = Vec::new();

    for _i in 0..height {
        rows.push(create_default_row(width));
    }

    return rows;
}

fn create_default_row(length: u8) -> Vec<Color> {
    let mut row = Vec::new();

    for _i in 0..length {
        row.push(Color::new(0.0, 0.0, 0.0));
    }

    return row;
}

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

                if row_string.chars().count() + 1 + value_string.chars().count()
                    < MAX_PPM_LINE_LENGTH as usize
                {
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

    return String::from(header + pixel_data.as_str());
}

fn convert_color_to_ppm_values(color: &Color) -> [u8; 3] {
    let r = convert_color_value_to_ppm_value(color.get_r());
    let b = convert_color_value_to_ppm_value(color.get_b());
    let g = convert_color_value_to_ppm_value(color.get_g());

    return [r, b, g];
}

fn convert_color_value_to_ppm_value(value: f64) -> u8 {
    let ppm_value = ((MAX_PPM_COLOR_VALUE as f64) * value) as f64;

    if ppm_value < 0.0 {
        return 0;
    }

    if ppm_value > MAX_PPM_COLOR_VALUE as f64 {
        return MAX_PPM_COLOR_VALUE;
    }

    return ppm_value.round() as u8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initializing_canvas() {
        let canvas = Canvas::new(10, 20);

        // The canvas has the correct dimensions...
        assert_eq!(canvas.get_width(), 10);
        assert_eq!(canvas.get_height(), 20);

        // ...and every pixel is black
        for row in canvas.get_rows().iter() {
            for pixel in row.iter() {
                assert_eq!(*pixel, Color::new(0.0, 0.0, 0.0))
            }
        }
    }

    #[test]
    fn writing_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, color);

        assert_eq!(canvas.get_pixel_at(2, 3), Color::new(1.0, 0.0, 0.0));
    }

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
