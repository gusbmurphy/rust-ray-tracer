use std::ops;

use crate::matrix::Matrix;
use crate::point::Point;
use crate::Tuple;

struct Translation {
    matrix: Matrix<4>,
}

impl Translation {
    pub fn new(x: f32, y: f32, z: f32) -> Translation {
        let matrix_values = [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Translation {
            matrix: Matrix::new(matrix_values),
        }
    }

    pub fn invert(&self) -> Result<Translation, &'static str> {
        let inversion_result = self.matrix.invert();

        match inversion_result {
            Ok(inverted_matrix) => {
                Ok(Translation { matrix: inverted_matrix })
            },
            Err(error) => {
                Err(error)
            }
        }
    }
}

impl ops::Mul<Point> for Translation {
    type Output = Point;

    fn mul(self, p: Point) -> Self::Output {
        let point_values = self.matrix * [p.get_x(), p.get_y(), p.get_z(), p.get_w()];
        Point::new(point_values[0], point_values[1], point_values[2])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiplying_point_by_a_translation() {
        let translation = Translation::new(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);

        let result = translation * point;
        let expected = Point::new(2.0, 1.0, 7.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_point_by_the_inverse_of_a_translation() {
        let translation = Translation::new(5.0, -3.0, 2.0);
        let inverse = translation.invert().unwrap();
        let point = Point::new(-3.0, 4.0, 5.0);

        let result = inverse * point;
        let expected = Point::new(-8.0, 7.0, 3.0);

        assert_eq!(result, expected);
    }
}
