pub use crate::prelude::*;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const S: usize> {
    values: [[f32; S]; S],
}

impl<const S: usize> Matrix<S> {
    pub fn new(values: [[f32; S]; S]) -> Self {
        Matrix { values }
    }

    pub fn get_row(&self, row: usize) -> &[f32; S] {
        &self.values[row]
    }

    pub fn get_column(&self, column: usize) -> [f32; S] {
        let mut column_values = [0.0f32; S];

        for row in 0..S {
            column_values[row] = self.values[row][column];
        }

        column_values
    }

    pub fn transpose(&self) -> Matrix<S> {
        let mut result_values = [[0.0f32; S]; S];

        for column in 0..S {
            for row in 0..S {
                result_values[row][column] = self.values[column][row];
            }
        }

        Matrix {
            values: result_values,
        }
    }

    pub fn set_value(&mut self, at_col: usize, at_row: usize, value: f32) {
        self.values[at_row][at_col] = value;
    }
}

impl<const S: usize> PartialEq for Matrix<S> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..S {
            for column in 0..S {
                if !close_enough(&self.values[row][column], &other.values[row][column]) {
                    return false;
                }
            }
        }

        true
    }
}

pub const IDENTITY_MATRIX: Matrix<4> = Matrix {
    values: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Matrix<4> {
    pub fn get_submatrix(&self, row_to_drop: usize, column_to_drop: usize) -> Matrix<3> {
        const SUBMATRIX_SIZE: usize = 3;
        let mut values = [[0.0f32; SUBMATRIX_SIZE]; SUBMATRIX_SIZE];

        let mut column_shift = 0;

        for column in 0..SUBMATRIX_SIZE {
            if column == column_to_drop {
                column_shift += 1;
            }

            let mut row_shift = 0;
            for row in 0..SUBMATRIX_SIZE {
                if row == row_to_drop {
                    row_shift += 1;
                }

                values[row][column] = self.values[row + row_shift][column + column_shift]
            }
        }

        Matrix { values }
    }

    fn calculate_minor_at(&self, row: usize, column: usize) -> f32 {
        let sub_matrix = self.get_submatrix(row, column);
        sub_matrix.calculate_determinant()
    }

    fn calculate_cofactor_at(&self, row: usize, column: usize) -> f32 {
        let minor = self.calculate_minor_at(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn calculate_determinant(&self) -> f32 {
        let mut determinant = 0.0f32;

        const ROW_INDEX_TO_USE: usize = 0;
        let row_to_use = self.get_row(ROW_INDEX_TO_USE);

        for column in 0..4 {
            let value = row_to_use[column];
            let cofactor = self.calculate_cofactor_at(ROW_INDEX_TO_USE, column);

            determinant += value * cofactor;
        }

        determinant
    }

    // TODO: The fact that this returns a Result kind of complicates things,
    // I wonder if there's a way that we can just avoid having Matrices with
    // determinants of 0 all together? Is there any use for them?
    pub fn invert(&self) -> Result<Matrix<4>, &'static str> {
        let determinant = self.calculate_determinant();

        if determinant == 0f32 {
            return Err("Matrix cannot be inverted.");
        }

        let mut cofactor_matrix = Matrix {
            values: [[0.0; 4]; 4],
        };

        for row in 0..4 {
            for column in 0..4 {
                cofactor_matrix.values[row][column] = self.calculate_cofactor_at(row, column);
            }
        }

        let transposed_cofactor_matrix = cofactor_matrix.transpose();

        let mut inverted_matrix = Matrix {
            values: [[0.0; 4]; 4],
        };

        for row in 0..4 {
            for column in 0..4 {
                inverted_matrix.values[row][column] =
                    transposed_cofactor_matrix.values[row][column] / determinant;
            }
        }

        Ok(inverted_matrix)
    }
}

impl<const S: usize> Mul<Matrix<S>> for Matrix<S> {
    type Output = Matrix<S>;

    fn mul(self, rhs: Matrix<S>) -> Self::Output {
        let mut result_values = [[0.0; S]; S];

        for column_index in 0..S {
            for row_index in 0..S {
                let mut value = 0.0;

                for i in 0..S {
                    value += self.get_row(row_index)[i] * rhs.get_column(column_index)[i];
                }

                result_values[row_index][column_index] = value;
            }
        }

        Matrix {
            values: result_values,
        }
    }
}

impl<const S: usize> Mul<[f32; S]> for Matrix<S> {
    type Output = [f32; S];

    fn mul(self, rhs: [f32; S]) -> Self::Output {
        let mut result = [0.0f32; S];

        for row in 0..S {
            for column in 0..S {
                result[row] += self.get_row(row)[column] * rhs[column];
            }
        }

        result
    }
}

impl Mul<Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let result_array = self * rhs.to_array();
        return Vector::new(result_array[0], result_array[1], result_array[2]);
    }
}

impl Matrix<3> {
    fn get_submatrix(&self, row_to_drop: usize, column_to_drop: usize) -> Matrix<2> {
        let mut values = [[0.0f32; 2]; 2];

        let mut column_shift = 0;

        for column in 0..2 {
            if column == column_to_drop {
                column_shift += 1;
            }

            let mut row_shift = 0;
            for row in 0..2 {
                if row == row_to_drop {
                    row_shift += 1;
                }

                values[row][column] = self.values[row + row_shift][column + column_shift]
            }
        }

        Matrix { values }
    }

    fn calculate_minor_at(&self, row: usize, column: usize) -> f32 {
        let sub_matrix = self.get_submatrix(row, column);
        sub_matrix.calculate_determinant()
    }

    fn calculate_cofactor_at(&self, row: usize, column: usize) -> f32 {
        let minor = self.calculate_minor_at(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn calculate_determinant(&self) -> f32 {
        let mut determinant = 0.0f32;

        const ROW_INDEX_TO_USE: usize = 0;
        let row_to_use = self.get_row(ROW_INDEX_TO_USE);

        for column in 0..3 {
            let value = row_to_use[column];
            let cofactor = self.calculate_cofactor_at(ROW_INDEX_TO_USE, column);

            determinant += value * cofactor;
        }

        determinant
    }
}

impl Matrix<2> {
    pub fn calculate_determinant(&self) -> f32 {
        self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
                    result.get_column(column)[row],
                    expected.get_column(column)[row]
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

    #[test]
    fn determinant_of_2_by_2_matrix() {
        let matrix = Matrix {
            values: [[1.0, 5.0], [-3.0, 2.0]],
        };

        let result = matrix.calculate_determinant();

        assert_eq!(result, 17.0)
    }

    #[test]
    fn getting_submatrix_of_3_by_3_matrix() {
        let matrix = Matrix {
            values: [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]],
        };

        let submatrix = matrix.get_submatrix(0, 2);
        let expected = Matrix {
            values: [[-3.0, 2.0], [0.0, 6.0]],
        };

        assert_eq!(submatrix, expected)
    }

    #[test]
    fn getting_submatrix_of_4_by_4_matrix() {
        let matrix = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [9.0, -3.0, 7.0, 2.0],
                [-32.0, 7.0, 10.0, -2.0],
                [-8.0, 13.0, 1.0, 9.0],
            ],
        };

        let submatrix = matrix.get_submatrix(2, 1);

        let expected = Matrix {
            values: [[1.0, 3.0, 4.0], [9.0, 7.0, 2.0], [-8.0, 1.0, 9.0]],
        };

        assert_eq!(submatrix, expected)
    }

    #[test]
    fn calculating_minor_of_3_by_3_matrix() {
        let matrix = Matrix {
            values: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert_eq!(matrix.calculate_minor_at(1, 0), 25.0)
    }

    #[test]
    fn calculating_cofactor_of_3_by_3_matrix() {
        let matrix = Matrix {
            values: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert_eq!(matrix.calculate_cofactor_at(1, 0), -25.0);
        assert_eq!(matrix.calculate_cofactor_at(2, 0), -35.0);
    }

    #[test]
    fn getting_rows_and_columns_of_matrices() {
        let four_matrix = Matrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [9.0, -3.0, 7.0, 2.0],
                [-32.0, 7.0, 10.0, -2.0],
                [-8.0, 13.0, 1.0, 9.0],
            ],
        };
        assert_eq!(four_matrix.get_row(2).to_owned(), [-32.0, 7.0, 10.0, -2.0]);
        assert_eq!(four_matrix.get_column(1), [2.0, -3.0, 7.0, 13.0]);

        let three_matrix = Matrix {
            values: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };
        assert_eq!(three_matrix.get_row(1).to_owned(), [2.0, -1.0, -7.0]);
        assert_eq!(three_matrix.get_column(0), [3.0, 2.0, 6.0]);

        let two_matrix = Matrix {
            values: [[1.0, 5.0], [-3.0, 2.0]],
        };
        assert_eq!(two_matrix.get_row(1).to_owned(), [-3.0, 2.0]);
        assert_eq!(two_matrix.get_column(1), [5.0, 2.0]);
    }

    #[test]
    fn calculating_determinant_of_3_by_3_matrix() {
        let matrix = Matrix {
            values: [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]],
        };

        assert_eq!(matrix.calculate_determinant(), -196.0);
    }

    #[test]
    fn calculating_determinant_of_4_by_4_matrix() {
        let matrix = Matrix {
            values: [
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ],
        };

        assert_eq!(matrix.calculate_determinant(), -4071.0);
    }

    #[test]
    fn inverting_non_invertable_matrix() {
        // A matrix whose determinant is 0 is not invertable.
        let matrix = Matrix {
            values: [
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };

        let result = matrix.invert();
        assert!(result.is_err());
    }

    #[test]
    fn inverting_4_by_4_matrix() {
        let matrix = Matrix {
            values: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ],
        };

        let expected_result = Matrix {
            values: [
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ],
        };

        let result = matrix.invert().unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let matrix_a = Matrix {
            values: [
                [3.0, -9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, -2.0, 0.0, 5.0],
            ],
        };

        let matrix_b = Matrix {
            values: [
                [8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0],
                [6.0, -2.0, 0.0, 5.0],
            ],
        };

        let product = matrix_a * matrix_b;

        let product_of_product_and_inverse = product * matrix_b.invert().unwrap();

        assert_eq!(product_of_product_and_inverse, matrix_a);
    }
}
