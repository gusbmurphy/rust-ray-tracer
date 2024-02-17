use std::ops::Mul;

#[derive(PartialEq, Debug, Clone, Copy)]
struct FourByFourMatrix {
    values: [[f32; 4]; 4],
}

const IDENTITY_MATRIX: FourByFourMatrix = FourByFourMatrix {
    values: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl FourByFourMatrix {
    pub fn get_value(&self, index: (usize, usize)) -> Option<&f32> {
        self.values.get(index.0).and_then(|row| row.get(index.1))
    }

    pub fn transpose(&self) -> FourByFourMatrix {
        let mut result_values = [[0.0f32; 4]; 4];

        for column in 0..4 {
            for row in 0..4 {
                result_values[row][column] = self.values[column][row];
            }
        }

        FourByFourMatrix {
            values: result_values,
        }
    }

    pub fn get_submatrix(&self, row_to_drop: usize, column_to_drop: usize) -> ThreeByThreeMatrix {
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

        ThreeByThreeMatrix { values }
    }
}

// TODO: Handle multiplication of different sized Matrixes handle better.
impl Mul<FourByFourMatrix> for FourByFourMatrix {
    type Output = FourByFourMatrix;

    fn mul(self, rhs: FourByFourMatrix) -> Self::Output {
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

        return FourByFourMatrix {
            values: result_values,
        };
    }
}

impl Mul<[f32; 4]> for FourByFourMatrix {
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

#[derive(Debug, PartialEq)]
struct ThreeByThreeMatrix {
    values: [[f32; 3]; 3],
}

impl ThreeByThreeMatrix {
    fn get_submatrix(&self, row_to_drop: usize, column_to_drop: usize) -> TwoByTwoMatrix {
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

        TwoByTwoMatrix { values }
    }
}

#[derive(Debug, PartialEq)]
struct TwoByTwoMatrix {
    values: [[f32; 2]; 2],
}

impl TwoByTwoMatrix {
    pub fn calculate_determinate(&self) -> f32 {
        self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn building_4_by_4_matrix() {
        let matrix = FourByFourMatrix {
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
        let matrix = FourByFourMatrix {
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
        let matrix1 = FourByFourMatrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        let matrix2 = FourByFourMatrix {
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
        let matrix1 = FourByFourMatrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        let matrix2 = FourByFourMatrix {
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
        let matrix1 = FourByFourMatrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };

        let matrix2 = FourByFourMatrix {
            values: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ],
        };

        let result = matrix1 * matrix2;
        let expected = FourByFourMatrix {
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
        let matrix = FourByFourMatrix {
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
        let matrix = FourByFourMatrix {
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
        let matrix = FourByFourMatrix {
            values: [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ],
        };

        let transposed_matrix = matrix.transpose();
        let expected_result = FourByFourMatrix {
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
    fn determinate_of_2_by_2_matrix() {
        let matrix = TwoByTwoMatrix {
            values: [[1.0, 5.0], [-3.0, 2.0]],
        };

        let result = matrix.calculate_determinate();

        assert_eq!(result, 17.0)
    }

    #[test]
    fn getting_submatrix_of_3_by_3_matrix() {
        let matrix = ThreeByThreeMatrix {
            values: [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]],
        };

        let submatrix = matrix.get_submatrix(0, 2);
        let expected = TwoByTwoMatrix {
            values: [[-3.0, 2.0], [0.0, 6.0]],
        };

        assert_eq!(submatrix, expected)
    }

    #[test]
    fn getting_submatrix_of_4_by_4_matrix() {
        let matrix = FourByFourMatrix {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [9.0, -3.0, 7.0, 2.0],
                [-32.0, 7.0, 10.0, -2.0],
                [-8.0, 13.0, 1.0, 9.0],
            ],
        };

        let submatrix = matrix.get_submatrix(2, 1);

        let expected = ThreeByThreeMatrix {
            values: [[1.0, 3.0, 4.0], [9.0, 7.0, 2.0], [-8.0, 1.0, 9.0]],
        };

        assert_eq!(submatrix, expected)
    }
}