struct Matrix {
    values: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(values: Vec<Vec<f32>>) -> Self {
        return Matrix { values };
    }

    pub fn get_value(&self, index: (usize, usize)) -> Option<&f32> {
        return Some(self.values.get(index.0).unwrap().get(index.1).unwrap());
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

        assert_eq!(matrix.get_value((0, 0)).unwrap().to_owned(), 1.0);
        assert_eq!(matrix.get_value((0, 3)).unwrap().to_owned(), 4.0);
        assert_eq!(matrix.get_value((1, 0)).unwrap().to_owned(), 5.5);
        assert_eq!(matrix.get_value((1, 2)).unwrap().to_owned(), 7.5);
        assert_eq!(matrix.get_value((2, 2)).unwrap().to_owned(), 11.0);
        assert_eq!(matrix.get_value((3, 0)).unwrap().to_owned(), 13.5);
        assert_eq!(matrix.get_value((3, 2)).unwrap().to_owned(), 15.5);
    }
}
