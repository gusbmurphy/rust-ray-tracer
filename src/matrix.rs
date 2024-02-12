use std::ops::Mul;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Matrix {
    values: [[f32; 4]; 4],
}

const IDENTITY_MATRIX: Matrix = Matrix {
    values: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Matrix {
    pub fn get_value(&self, index: (usize, usize)) -> Option<&f32> {
        self.values.get(index.0).and_then(|row| row.get(index.1))
    }

    pub fn transpose(&self) -> Matrix {
        let mut result_values = [[0.0f32; 4]; 4];

        for column in 0..4 {
            for row in 0..4 {
                result_values[row][column] = self.values[column][row];
            }
        }

        Matrix {
            values: result_values,
        }
    }
}

// TODO: Handle multiplication of different sized Matrixes handle better.
impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let size = self.values.len();
        let mut result_values = [[0.0; 4]; 4];

        for column_index in 0..size {
            for row_index in 0..size {
                let mut value = 0.0;

                for i in 0..size {
                    value += self.get_value((row_index, i)).unwrap()
                        * rhs.get_value((i, column_index)).unwrap();
                }

                result_values[row_index][column_index] = value;
            }
        }

        return Matrix {
            values: result_values,
        };
    }
}

impl Mul<[f32; 4]> for Matrix {
    type Output = [f32; 4];

    fn mul(self, rhs: [f32; 4]) -> Self::Output {
        let mut result = [0.0f32; 4];

        for row in 0..4 {
            for column in 0..4 {
                // TODO: So much unwrapping happening...
                result[row] += self.get_value((row, column)).unwrap() * rhs[column];
            }
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn building_4_by_4_matrix() {
        let matrix = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        assert_eq!(matrix.get_value((0, 0)).unwrap().to_owned(), 1.0);
        assert_eq!(matrix.get_value((0, 3)).unwrap().to_owned(), 4.0);
        assert_eq!(matrix.get_value((1, 0)).unwrap().to_owned(), 5.5);
        assert_eq!(matrix.get_value((1, 2)).unwrap().to_owned(), 7.5);
        assert_eq!(matrix.get_value((2, 2)).unwrap().to_owned(), 11.0);
        assert_eq!(matrix.get_value((3, 0)).unwrap().to_owned(), 13.5);
        assert_eq!(matrix.get_value((3, 2)).unwrap().to_owned(), 15.5);
    }

    #[test]
    fn get_value_returns_nothing_outside_of_bounds() {
        let matrix = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        assert_eq!(matrix.get_value((4, 0)), None);
        assert_eq!(matrix.get_value((0, 4)), None);
    }

    #[test]
    fn matrix_equality() {
        let matrix1 = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        let matrix2 = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        assert!(matrix1 == matrix2)
    }

    #[test]
    fn matrix_non_equality() {
        let matrix1 = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        let matrix2 = Matrix {
            values: [
                [9999.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        assert!(matrix1 != matrix2)
    }

    #[test]
    fn matrix_multiplication() {
        let matrix1 = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };

        let matrix2 = Matrix {
            values: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ],
        };

        let result = matrix1 * matrix2;
        let expected = Matrix {
            values: [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ],
        };

        for column in 0..4 {
            for row in 0..4 {
                assert_eq!(
                    result.get_value((column, row)),
                    expected.get_value((column, row))
                )
            }
        }
    }

    #[test]
    fn matrix_multiplication_by_tuple() {
        let matrix = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let tuple = [1.0, 2.0, 3.0, 1.0];

        let result = matrix * tuple;

        assert_eq!(result, [18.0, 24.0, 33.0, 1.0]);
    }

    #[test]
    fn multiplication_by_identity_matrix_returns_original_matrix() {
        let matrix = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let result = matrix * IDENTITY_MATRIX;

        assert_eq!(result, matrix)
    }

    #[test]
    fn matrix_transposition() {
        let matrix = Matrix {
            values: [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ],
        };

        let transposed_matrix = matrix.transpose();
        let expected_result = Matrix {
            values: [
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ],
        };

        assert_eq!(transposed_matrix, expected_result)
    }

    #[test]
    fn identity_matrix_transposition() {
        let result = IDENTITY_MATRIX.transpose();

        assert_eq!(result, IDENTITY_MATRIX)
    }
}
