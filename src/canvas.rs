use crate::color::Color;

struct Canvas {
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
}
