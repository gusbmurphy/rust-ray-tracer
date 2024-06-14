use std::ops;

use super::matrix::*;
use super::ray::Ray;
use super::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    matrix: Matrix<4>,
}

impl Transform {
    pub fn new(matrix: Matrix<4>) -> Self {
        Transform { matrix }
    }

    pub fn new_view(from: Point, to: Point, approximate_up: Vector) -> Self {
        let forward = (to - from).normalize();
        let left = cross(&forward, &approximate_up.normalize());
        let true_up = cross(&left, &forward);

        let orientation = Transform::new(Matrix::new([
            [left.get_x(), left.get_y(), left.get_z(), 0.0],
            [true_up.get_x(), true_up.get_y(), true_up.get_z(), 0.0],
            [-forward.get_x(), -forward.get_y(), -forward.get_z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));

        let orientation_moved_into_place =
            orientation * Transform::new_translation(-from.get_x(), -from.get_y(), -from.get_z());

        return orientation_moved_into_place;
    }

    pub fn new_translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(3, 0, x);
        matrix.set_value(3, 1, y);
        matrix.set_value(3, 2, z);

        Transform { matrix }
    }

    pub fn new_scaling(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(0, 0, x);
        matrix.set_value(1, 1, y);
        matrix.set_value(2, 2, z);

        Transform { matrix }
    }

    pub fn new_x_rotation(radians: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(1, 1, radians.cos());
        matrix.set_value(2, 1, -radians.sin());
        matrix.set_value(1, 2, radians.sin());
        matrix.set_value(2, 2, radians.cos());

        Transform { matrix }
    }

    pub fn new_y_rotation(radians: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(0, 0, radians.cos());
        matrix.set_value(0, 2, -radians.sin());
        matrix.set_value(2, 0, radians.sin());
        matrix.set_value(2, 2, radians.cos());

        Transform { matrix }
    }

    pub fn new_z_rotation(radians: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(0, 0, radians.cos());
        matrix.set_value(1, 0, -radians.sin());
        matrix.set_value(0, 1, radians.sin());
        matrix.set_value(1, 1, radians.cos());

        Transform { matrix }
    }

    pub fn new_shearing(
        x_to_y: f32,
        x_to_z: f32,
        y_to_x: f32,
        y_to_z: f32,
        z_to_x: f32,
        z_to_y: f32,
    ) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(1, 0, x_to_y);
        matrix.set_value(2, 0, x_to_z);
        matrix.set_value(0, 1, y_to_x);
        matrix.set_value(2, 1, y_to_z);
        matrix.set_value(0, 2, z_to_x);
        matrix.set_value(1, 2, z_to_y);

        Transform { matrix }
    }

    pub fn invert(&self) -> Result<Transform, &'static str> {
        let inversion_result = self.matrix.invert();

        match inversion_result {
            Ok(inverted_matrix) => Ok(Transform {
                matrix: inverted_matrix,
            }),
            Err(error) => Err(error),
        }
    }

    pub fn get_matrix(&self) -> &Matrix<4> {
        &self.matrix
    }
}

impl<T: Tuple> ops::Mul<T> for Transform {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let point_values = self.matrix * [rhs.get_x(), rhs.get_y(), rhs.get_z(), rhs.get_w()];
        T::new(point_values[0], point_values[1], point_values[2])
    }
}

impl ops::Mul<&Ray> for Transform {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        let new_origin = self * rhs.get_origin().to_owned();
        let new_direction = self * rhs.get_direction().to_owned();
        Ray::new(new_origin, new_direction)
    }
}

impl ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            matrix: self.matrix * rhs.matrix,
        }
    }
}

impl PartialEq<Matrix<4>> for Transform {
    fn eq(&self, other: &Matrix<4>) -> bool {
        self.matrix == other.to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn multiplying_point_by_a_translation() {
        let translation = Transform::new_translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);

        let result = translation * point;
        let expected = Point::new(2.0, 1.0, 7.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_point_by_the_inverse_of_a_translation() {
        let translation = Transform::new_translation(5.0, -3.0, 2.0);
        let inverse = translation.invert().unwrap();
        let point = Point::new(-3.0, 4.0, 5.0);

        let result = inverse * point;
        let expected = Point::new(-8.0, 7.0, 3.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let translation = Transform::new_translation(5.0, -3.0, 2.0);
        let vector = Vector::new(-3.0, 4.0, 5.0);

        let result = translation * vector;

        assert_eq!(result, vector);
    }

    #[test]
    fn scaling_a_point() {
        let scaling = Transform::new_scaling(2.0, 3.0, 4.0);
        let point = Point::new(-4.0, 6.0, 8.0);

        let result = scaling * point;
        let expected = Point::new(-8.0, 18.0, 32.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn scaling_a_vector() {
        let scaling = Transform::new_scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);

        let result = scaling * vector;
        let expected = Vector::new(-8.0, 18.0, 32.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling() {
        let scaling = Transform::new_scaling(2.0, 3.0, 4.0);
        let inversion = scaling.invert().unwrap();

        let vector = Vector::new(-4.0, 6.0, 8.0);

        let result = inversion * vector;
        let expected = Vector::new(-2.0, 2.0, 2.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let scaling = Transform::new_scaling(-1.0, 1.0, 1.0);
        let point = Point::new(2.0, 3.0, 4.0);

        let result = scaling * point;
        let expected = Point::new(-2.0, 3.0, 4.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::new_x_rotation(PI / 4.0);
        let full_quarter = Transform::new_x_rotation(PI / 2.0);

        assert_eq!(
            half_quarter * point,
            Point::new(0.0, 2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_opposite_direction() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::new_x_rotation(PI / 4.0);
        let inverse = half_quarter.invert().unwrap();

        assert_eq!(
            inverse * point,
            Point::new(0.0, 2.0f32.sqrt() / 2.0, -2.0f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let point = Point::new(0.0, 0.0, 1.0);

        let half_quarter = Transform::new_y_rotation(PI / 4.0);
        let full_quarter = Transform::new_y_rotation(PI / 2.0);

        assert_eq!(
            half_quarter * point,
            Point::new(2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::new_z_rotation(PI / 4.0);
        let full_quarter = Transform::new_z_rotation(PI / 2.0);

        assert_eq!(
            half_quarter * point,
            Point::new(-2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * point, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transform::new_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(5.0, 3.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transform::new_shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(6.0, 3.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transform::new_shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(2.0, 5.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transform::new_shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(2.0, 7.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transform::new_shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        assert_eq!(shearing * point, Point::new(2.0, 3.0, 6.0))
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transform::new_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        assert_eq!(shearing * point, Point::new(2.0, 3.0, 7.0))
    }

    #[test]
    fn a_ray_is_translatable() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(0.0, 1.0, 0.0);
        let ray = Ray::new(point, vector);

        let translation = Transform::new_translation(3.0, 4.0, 5.0);

        let result = translation * &ray;

        assert_eq!(
            result,
            Ray::new(Point::new(4.0, 6.0, 8.0), Vector::new(0.0, 1.0, 0.0))
        )
    }

    #[test]
    fn a_ray_is_scalable() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(0.0, 1.0, 0.0);
        let ray = Ray::new(point, vector);

        let scaling = Transform::new_scaling(2.0, 3.0, 4.0);

        let result = scaling * &ray;

        assert_eq!(
            result,
            Ray::new(Point::new(2.0, 6.0, 12.0), Vector::new(0.0, 3.0, 0.0))
        )
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = Point::new(1.0, 0.0, 1.0);

        let rotation = Transform::new_x_rotation(PI / 2.0);
        let scaling = Transform::new_scaling(5.0, 5.0, 5.0);
        let translation = Transform::new_translation(10.0, 5.0, 7.0);

        let point_after_rotation = rotation * point;
        assert_eq!(point_after_rotation, Point::new(1.0, -1.0, 0.0));

        let point_after_scaling = scaling * point_after_rotation;
        assert_eq!(point_after_scaling, Point::new(5.0, -5.0, 0.0));

        let point_after_translation = translation * point_after_scaling;
        assert_eq!(point_after_translation, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_tranformations_must_be_applied_in_reverse_order() {
        let point = Point::new(1.0, 0.0, 1.0);

        let rotation = Transform::new_x_rotation(PI / 2.0);
        let scaling = Transform::new_scaling(5.0, 5.0, 5.0);
        let translation = Transform::new_translation(10.0, 5.0, 7.0);

        let combined_transformation = translation * scaling * rotation;

        assert_eq!(combined_transformation * point, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn default_view_transformation_is_the_identity_matrix() {
        let center = Point::new(0.0, 0.0, 0.0);
        let point_to_look_at = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let view_transform = Transform::new_view(center, point_to_look_at, up);

        assert_eq!(view_transform, IDENTITY_MATRIX);
    }

    #[test]
    fn a_view_transformation_looking_in_the_positive_z_direction_is_a_reflection() {
        let center = Point::new(0.0, 0.0, 0.0);
        let point_to_look_at = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let view_transform = Transform::new_view(center, point_to_look_at, up);

        assert_eq!(view_transform, Transform::new_scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn a_view_transformation_is_the_same_as_a_translation_in_the_opposite_direction() {
        let center = Point::new(0.0, 0.0, 8.0);
        let point_to_look_at = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let view_transform = Transform::new_view(center, point_to_look_at, up);

        assert_eq!(view_transform, Transform::new_translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn some_arbitrary_view_transformation_is_constructed_correctly() {
        let center = Point::new(1.0, 3.0, 2.0);
        let point_to_look_at = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);

        let view_transform = Transform::new_view(center, point_to_look_at, up);

        let expected_matrix = Matrix::new([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(view_transform, expected_matrix);
    }
}
