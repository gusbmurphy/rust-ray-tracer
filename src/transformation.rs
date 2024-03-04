use std::ops;

use crate::matrix::Matrix;
use crate::point::Point;
use crate::Tuple;

struct Transformation {
    matrix: Matrix<4>,
}

impl Transformation {
    pub fn new_translation(x: f32, y: f32, z: f32) -> Self {
        let matrix_values = [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Transformation {
            matrix: Matrix::new(matrix_values),
        }
    }

    pub fn invert(&self) -> Result<Transformation, &'static str> {
        let inversion_result = self.matrix.invert();

        match inversion_result {
            Ok(inverted_matrix) => {
                Ok(Transformation { matrix: inverted_matrix })
            },
            Err(error) => {
                Err(error)
            }
        }
    }
}

impl<T: Tuple> ops::Mul<T> for Transformation {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let point_values = self.matrix * [rhs.get_x(), rhs.get_y(), rhs.get_z(), rhs.get_w()];
        T::new(point_values[0], point_values[1], point_values[2])
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

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
}
