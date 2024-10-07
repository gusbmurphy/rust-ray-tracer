use super::Canvas;

pub fn create_png(canvas: Canvas, path: &str) -> Result<(), image::ImageError> {
    let mut image = image::RgbImage::new(*canvas.width(), *canvas.height());

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = canvas.pixel_at(x as usize, y as usize);
        *pixel = image::Rgb(color.to_rgb())
    }

    image.save(path)
}
