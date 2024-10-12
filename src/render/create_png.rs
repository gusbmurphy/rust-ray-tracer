use super::Canvas;
use image::ImageBuffer;
use image::Rgb;

pub fn create_png(canvas: Canvas) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = image::RgbImage::new(*canvas.width(), *canvas.height());

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = canvas.pixel_at(x as usize, y as usize);
        *pixel = image::Rgb(color.to_rgb())
    }

    image
}
