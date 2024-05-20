use std::ops;

use super::matrix::*;
use super::ray::Ray;
use super::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Transformation {
    matrix: Matrix<4>,
}

impl Transformation {
    pub fn new(matrix: Matrix<4>) -> Self {
        Transformation { matrix }
    }

    pub fn new_translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(3, 0, x);
        matrix.set_value(3, 1, y);
        matrix.set_value(3, 2, z);

        Transformation { matrix }
    }

    pub fn new_scaling(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(0, 0, x);
        matrix.set_value(1, 1, y);
        matrix.set_value(2, 2, z);

        Transformation { matrix }
    }

    pub fn new_x_rotation(radians: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(1, 1, radians.cos());
        matrix.set_value(2, 1, -radians.sin());
        matrix.set_value(1, 2, radians.sin());
        matrix.set_value(2, 2, radians.cos());

        Transformation { matrix }
    }

    pub fn new_y_rotation(radians: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(0, 0, radians.cos());
        matrix.set_value(0, 2, -radians.sin());
        matrix.set_value(2, 0, radians.sin());
        matrix.set_value(2, 2, radians.cos());

        Transformation { matrix }
    }

    pub fn new_z_rotation(radians: f32) -> Self {
        let mut matrix = IDENTITY_MATRIX;

        matrix.set_value(0, 0, radians.cos());
        matrix.set_value(1, 0, -radians.sin());
        matrix.set_value(0, 1, radians.sin());
        matrix.set_value(1, 1, radians.cos());

        Transformation { matrix }
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

        Transformation { matrix }
    }

    pub fn invert(&self) -> Result<Transformation, &'static str> {
        let inversion_result = self.matrix.invert();

        match inversion_result {
            Ok(inverted_matrix) => Ok(Transformation {
                matrix: inverted_matrix,
            }),
            Err(error) => Err(error),
        }
    }

    pub fn get_matrix(&self) -> &Matrix<4> {
        &self.matrix
    }
}

impl<T: Tuple> ops::Mul<T> for Transformation {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let point_values = self.matrix * [rhs.get_x(), rhs.get_y(), rhs.get_z(), rhs.get_w()];
        T::new(point_values[0], point_values[1], point_values[2])
    }
}

impl ops::Mul<Ray> for Transformation {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        let new_origin = self * rhs.get_origin().to_owned();
        let new_direction = self * rhs.get_direction().to_owned();
        Ray::new(new_origin, new_direction)
    }
}

impl ops::Mul<Transformation> for Transformation {
    type Output = Transformation;

    fn mul(self, rhs: Transformation) -> Self::Output {
        Transformation {
            matrix: self.matrix * rhs.matrix,
        }
    }
}

impl PartialEq<Matrix<4>> for Transformation {
    fn eq(&self, other: &Matrix<4>) -> bool {
        self.matrix == other.to_owned()
    }    
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use crate::bones::ray::Ray;

    use super::*;

    #[test]
    fn multiplying_point_by_a_translation() {
        let translation = Transformation::new_translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);

        let result = translation * point;
        let expected = Point::new(2.0, 1.0, 7.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_point_by_the_inverse_of_a_translation() {
        let translation = Transformation::new_translation(5.0, -3.0, 2.0);
        let inverse = translation.invert().unwrap();
        let point = Point::new(-3.0, 4.0, 5.0);

        let result = inverse * point;
        let expected = Point::new(-8.0, 7.0, 3.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let translation = Transformation::new_translation(5.0, -3.0, 2.0);
        let vector = Vector::new(-3.0, 4.0, 5.0);

        let result = translation * vector;

        assert_eq!(result, vector);
    }

    #[test]
    fn scaling_a_point() {
        let scaling = Transformation::new_scaling(2.0, 3.0, 4.0);
        let point = Point::new(-4.0, 6.0, 8.0);

        let result = scaling * point;
        let expected = Point::new(-8.0, 18.0, 32.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn scaling_a_vector() {
        let scaling = Transformation::new_scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);

        let result = scaling * vector;
        let expected = Vector::new(-8.0, 18.0, 32.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling() {
        let scaling = Transformation::new_scaling(2.0, 3.0, 4.0);
        let inversion = scaling.invert().unwrap();

        let vector = Vector::new(-4.0, 6.0, 8.0);

        let result = inversion * vector;
        let expected = Vector::new(-2.0, 2.0, 2.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let scaling = Transformation::new_scaling(-1.0, 1.0, 1.0);
        let point = Point::new(2.0, 3.0, 4.0);

        let result = scaling * point;
        let expected = Point::new(-2.0, 3.0, 4.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transformation::new_x_rotation(PI / 4.0);
        let full_quarter = Transformation::new_x_rotation(PI / 2.0);

        assert_eq!(
            half_quarter * point,
            Point::new(0.0, 2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_opposite_direction() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transformation::new_x_rotation(PI / 4.0);
        let inverse = half_quarter.invert().unwrap();

        assert_eq!(
            inverse * point,
            Point::new(0.0, 2.0f32.sqrt() / 2.0, -2.0f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let point = Point::new(0.0, 0.0, 1.0);

        let half_quarter = Transformation::new_y_rotation(PI / 4.0);
        let full_quarter = Transformation::new_y_rotation(PI / 2.0);

        assert_eq!(
            half_quarter * point,
            Point::new(2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transformation::new_z_rotation(PI / 4.0);
        let full_quarter = Transformation::new_z_rotation(PI / 2.0);

        assert_eq!(
            half_quarter * point,
            Point::new(-2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * point, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transformation::new_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(5.0, 3.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transformation::new_shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(6.0, 3.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transformation::new_shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(2.0, 5.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transformation::new_shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        assert_eq!(shearing * point, Point::new(2.0, 7.0, 4.0))
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transformation::new_shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        assert_eq!(shearing * point, Point::new(2.0, 3.0, 6.0))
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let point = Point::new(2.0, 3.0, 4.0);
        let shearing = Transformation::new_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        assert_eq!(shearing * point, Point::new(2.0, 3.0, 7.0))
    }

    #[test]
    fn a_ray_is_translatable() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(0.0, 1.0, 0.0);
        let ray = Ray::new(point, vector);

        let translation = Transformation::new_translation(3.0, 4.0, 5.0);

        let result = translation * ray;

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

        let scaling = Transformation::new_scaling(2.0, 3.0, 4.0);

        let result = scaling * ray;

        assert_eq!(
            result,
            Ray::new(Point::new(2.0, 6.0, 12.0), Vector::new(0.0, 3.0, 0.0))
        )
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = Point::new(1.0, 0.0, 1.0);

        let rotation = Transformation::new_x_rotation(PI / 2.0);
        let scaling = Transformation::new_scaling(5.0, 5.0, 5.0);
        let translation = Transformation::new_translation(10.0, 5.0, 7.0);

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

        let rotation = Transformation::new_x_rotation(PI / 2.0);
        let scaling = Transformation::new_scaling(5.0, 5.0, 5.0);
        let translation = Transformation::new_translation(10.0, 5.0, 7.0);

        let combined_transformation = translation * scaling * rotation;

        assert_eq!(combined_transformation * point, Point::new(15.0, 0.0, 7.0));
    }
}
