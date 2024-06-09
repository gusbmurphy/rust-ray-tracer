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
            transform: Transform::new(IDENTITY_MATRIX),
        }
    }

    pub fn new_with_transform(
        horizontal_size: u32,
        vertical_size: u32,
        field_of_view: f32,
        transform: Transform,
    ) -> Self {
        Camera {
            horizontal_size,
            vertical_size,
            field_of_view,
            transform,
        }
    }

    pub fn get_pixel_size(&self) -> f32 {
        (self.get_half_width() * 2.0) / (self.horizontal_size as f32)
    }
    
    pub fn get_ray_for_pixel(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let x_offset = (pixel_x as f32 + 0.5) * self.get_pixel_size();
        let y_offset = (pixel_y as f32 + 0.5) * self.get_pixel_size();

        let world_x = self.get_half_width() - x_offset;
        let world_y = self.get_half_height() - y_offset;

        let origin = self.transform.invert().unwrap() * ORIGIN;

        let pixel_position = self.transform.invert().unwrap() * Point::new(world_x, world_y, -1.0);
        let direction =  (pixel_position - origin).normalize();

        Ray::new(origin, direction)
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

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray: Ray = camera.get_ray_for_pixel(100, 50);

        assert_eq!(ray.get_origin().to_owned(), ORIGIN);
        assert_eq!(ray.get_direction().to_owned(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray: Ray = camera.get_ray_for_pixel(0, 0);

        assert_eq!(ray.get_origin().to_owned(), ORIGIN);
        assert_eq!(ray.get_direction().to_owned(), Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let transform =
            Transform::new_y_rotation(PI / 4.0) * Transform::new_translation(0.0, -2.0, 5.0);
        let camera = Camera::new_with_transform(201, 101, PI / 2.0, transform);

        let ray: Ray = camera.get_ray_for_pixel(100, 50);

        assert_eq!(ray.get_origin().to_owned(), Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            ray.get_direction().to_owned(),
            Vector::new(2.0f32.sqrt() / 2.0, 0.0, -2.0f32.sqrt() / 2.0)
        );
    }
}
