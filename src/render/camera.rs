use crate::prelude::*;

struct Camera {
    horizontal_size: u32,
    vertical_size: u32,
    field_of_view: f32,
    transform: Transform,
}

impl Camera {
    pub fn new(horizontal_size: u32, vertical_size: u32, field_of_view: f32) -> Self {
        Camera {
            horizontal_size,
            vertical_size,
            field_of_view,
            transform: Transform::new(IDENTITY_MATRIX)
        }
    }

    pub fn get_pixel_size(&self) -> f32 {
        (self.get_half_width() * 2.0) / (self.horizontal_size as f32)
    }

    fn get_half_view(&self) -> f32 {
        (self.field_of_view / 2.0).tan()
    }

    fn get_half_width(&self) -> f32 {
        let aspect = self.get_aspect();

        if aspect >= 1.0 {
            return self.get_half_view();
        }

        return self.get_half_view() * aspect;
    }

    fn get_half_height(&self) -> f32 {
        let aspect: f32 = self.get_aspect();

        if aspect >= 1.0 {
            return self.get_half_view() / aspect;
        }

        return self.get_half_view();
    }

    fn get_aspect(&self) -> f32 {
        (self.horizontal_size as f32) / (self.vertical_size as f32)
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn the_default_transform_for_a_camera_is_the_identity_matrix() {
        let camera = Camera::new(160, 120, PI / 2.0);

        assert_eq!(camera.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas_is_correct() {
        let camera = Camera::new(200, 125, PI / 2.0);

        assert_eq!(camera.get_pixel_size(), 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas_is_correct() {
        let camera = Camera::new(125, 200, PI / 2.0);

        assert_eq!(camera.get_pixel_size(), 0.01);
    }
}
