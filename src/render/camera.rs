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
}
