use egui::emath::Numeric;
use mockall::automock;

use crate::prelude::*;
use crate::render::shading::shade_ray;

pub struct Camera<'l> {
    horizontal_size: u32,
    vertical_size: u32,
    total_pixels: u32,
    field_of_view: f64,
    transform: Transform,
    progress_listeners: Vec<&'l dyn RenderProgressListener>,
}

impl<'l> Camera<'l> {
    pub fn new(horizontal_size: u32, vertical_size: u32, field_of_view: f64) -> Self {
        Camera {
            horizontal_size,
            vertical_size,
            total_pixels: horizontal_size * vertical_size,
            field_of_view,
            transform: Transform::new(IDENTITY_MATRIX),
            progress_listeners: Vec::new(),
        }
    }

    pub fn new_with_transform(
        horizontal_size: u32,
        vertical_size: u32,
        field_of_view: f64,
        transform: Transform,
    ) -> Self {
        Camera {
            horizontal_size,
            vertical_size,
            total_pixels: horizontal_size * vertical_size,
            field_of_view,
            transform,
            progress_listeners: Vec::new(),
        }
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.horizontal_size, self.vertical_size);

        for x in 0..self.horizontal_size {
            for y in 0..self.vertical_size {
                let ray = self.get_ray_for_pixel(x, y);

                let color = shade_ray(&world, &ray);

                canvas.write_pixel(x as usize, y as usize, color);

                for listener in &self.progress_listeners {
                    listener.on_progress(self.current_completion(x, y));
                }
            }
        }

        return canvas;
    }

    fn current_completion(&self, x: u32, y: u32) -> f64 {
        let pixels_completed = x * self.horizontal_size + y + 1;
        pixels_completed.to_f64() / self.total_pixels.to_f64()
    }

    pub fn get_pixel_size(&self) -> f64 {
        (self.half_width() * 2.0) / (self.horizontal_size as f64)
    }

    pub fn get_ray_for_pixel(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let x_offset = (pixel_x as f64 + 0.5) * self.get_pixel_size();
        let y_offset = (pixel_y as f64 + 0.5) * self.get_pixel_size();

        let world_x = self.half_width() - x_offset;
        let world_y = self.half_height() - y_offset;

        let origin = self.transform.invert().unwrap() * ORIGIN;

        let pixel_position = self.transform.invert().unwrap() * Point::new(world_x, world_y, -1.0);
        let direction = (pixel_position - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn subscribe_to_progress(&mut self, listener: &'l dyn RenderProgressListener) {
        self.progress_listeners.push(listener);
    }

    fn half_view(&self) -> f64 {
        (self.field_of_view / 2.0).tan()
    }

    fn half_width(&self) -> f64 {
        let aspect = self.aspect();

        if aspect >= 1.0 {
            return self.half_view();
        }

        return self.half_view() * aspect;
    }

    fn half_height(&self) -> f64 {
        let aspect: f64 = self.aspect();

        if aspect >= 1.0 {
            return self.half_view() / aspect;
        }

        return self.half_view();
    }

    fn aspect(&self) -> f64 {
        (self.horizontal_size as f64) / (self.vertical_size as f64)
    }

    pub fn width(&self) -> &u32 {
        &self.horizontal_size
    }

    pub fn height(&self) -> &u32 {
        &self.vertical_size
    }

    pub fn fov(&self) -> &f64 {
        &self.field_of_view
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}

#[automock]
pub trait RenderProgressListener {
    fn on_progress(&self, completion: f64);
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use mockall::{predicate::eq, Sequence};

    use super::*;

    #[test]
    fn the_default_transform_for_a_camera_is_the_identity_matrix() {
        let camera = Camera::new(160, 120, PI / 2.0);

        assert_eq!(camera.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas_is_correct() {
        let camera = Camera::new(200, 125, PI / 2.0);

        assert!(close_enough(&camera.get_pixel_size(), &0.01));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas_is_correct() {
        let camera = Camera::new(125, 200, PI / 2.0);

        assert!(close_enough(&camera.get_pixel_size(), &0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray: Ray = camera.get_ray_for_pixel(100, 50);

        assert_eq!(ray.origin().to_owned(), ORIGIN);
        assert_eq!(ray.direction().to_owned(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.0);

        let ray: Ray = camera.get_ray_for_pixel(0, 0);

        assert_eq!(ray.origin().to_owned(), ORIGIN);
        assert_eq!(
            ray.direction().to_owned(),
            Vector::new(0.66519, 0.33259, -0.66851)
        );
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let transform = Transform::y_rotation(PI / 4.0) * Transform::translation(0.0, -2.0, 5.0);
        let camera = Camera::new_with_transform(201, 101, PI / 2.0, transform);

        let ray: Ray = camera.get_ray_for_pixel(100, 50);

        assert_eq!(ray.origin().to_owned(), Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            ray.direction().to_owned(),
            Vector::new(2.0f64.sqrt() / 2.0, 0.0, -2.0f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_has_the_correct_pixel_in_the_center() {
        let world = World::create_default();
        let camera_transform = Transform::view(
            Point::new(0.0, 0.0, -5.0),
            ORIGIN,
            Vector::new(0.0, 1.0, 0.0),
        );
        let camera = Camera::new_with_transform(11, 11, PI / 2.0, camera_transform);

        let canvas: Canvas = camera.render(world);

        assert_eq!(*canvas.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn when_rendering_progress_is_reported() {
        let mut listener = MockRenderProgressListener::new();
        let mut call_sequence = Sequence::new();

        let mut expect_progress_reported = |progress: f64| {
            listener
                .expect_on_progress()
                .once()
                .with(eq(progress))
                .in_sequence(&mut call_sequence)
                .return_const(());
        };

        expect_progress_reported(0.25);
        expect_progress_reported(0.5);
        expect_progress_reported(0.75);
        expect_progress_reported(1.0);

        let mut camera = Camera::new(2, 2, 10.0);
        camera.subscribe_to_progress(&listener);

        let world = World::create_default();
        camera.render(world);
    }
}
