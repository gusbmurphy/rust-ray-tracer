use crate::render::color::Color;

pub struct Canvas {
    width: u64,
    height: u64,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u64, height: u64) -> Self {
        Canvas {
            width,
            height,
            pixels: create_all_default_rows(width, height),
        }
    }

    pub fn get_width(&self) -> &u64 {
        &self.width
    }

    pub fn get_height(&self) -> &u64 {
        &self.height
    }

    pub fn get_rows(&self) -> &Vec<Vec<Color>> {
        &self.pixels
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn get_pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y][x]
    }
}

fn create_all_default_rows(width: u64, height: u64) -> Vec<Vec<Color>> {
    let mut rows = Vec::new();

    for _i in 0..height {
        rows.push(create_default_row(width));
    }

    rows
}

fn create_default_row(length: u64) -> Vec<Color> {
    let mut row = Vec::new();

    for _i in 0..length {
        row.push(Color::new(0.0, 0.0, 0.0));
    }

    row
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initializing_canvas() {
        let canvas = Canvas::new(10, 20);

        // The canvas has the correct dimensions...
        assert_eq!(*canvas.get_width(), 10);
        assert_eq!(*canvas.get_height(), 20);

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

        assert_eq!(*canvas.get_pixel_at(2, 3), Color::new(1.0, 0.0, 0.0));
    }
}
