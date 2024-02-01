use std::ops::Index;

struct Matrix {
    values: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(values: Vec<Vec<f32>>) -> Self {
        return Matrix { values };
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return self.values.get(index.0).unwrap().get(index.1).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn building_4_by_4_matrix() {
        let matrix = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 3)], 4.0);
        assert_eq!(matrix[(1, 0)], 5.5);
        assert_eq!(matrix[(1, 2)], 7.5);
        assert_eq!(matrix[(2, 2)], 11.0);
        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[(3, 2)], 15.5);
    }
}
