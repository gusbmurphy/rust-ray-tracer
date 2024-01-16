use std::ops;

#[derive(PartialEq, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    pub fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
        return self.y;
    }

    pub fn get_z(&self) -> f64 {
        return self.z;
    }

    pub fn get_w(&self) -> f64 {
        return self.w;
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Self::Output {
        return Tuple::new(
            self.get_x() + other.get_x(),
            self.get_y() + other.get_y(),
            self.get_z() + other.get_z(),
            self.get_w() + other.get_w(),
        );
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Self::Output {
        return Tuple::new(
            self.get_x() - other.get_x(),
            self.get_y() - other.get_y(),
            self.get_z() - other.get_z(),
            self.get_w() - other.get_w(),
        );
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        return Tuple::new(
            -self.get_x(),
            -self.get_y(),
            -self.get_z(),
            -self.get_w(),
        );
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        return Tuple::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs
            );
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        return Tuple::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w / rhs
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_1_for_w_is_a_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(tuple.get_x(), 4.3);
        assert_eq!(tuple.get_y(), -4.2);
        assert_eq!(tuple.get_z(), 3.1);
        assert_eq!(tuple.get_w(), 1.0);
        assert_eq!(tuple.is_point(), true);
        assert_eq!(tuple.is_vector(), false);
    }

    #[test]
    fn tuple_with_1_for_w_is_a_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(tuple.get_x(), 4.3);
        assert_eq!(tuple.get_y(), -4.2);
        assert_eq!(tuple.get_z(), 3.1);
        assert_eq!(tuple.get_w(), 0.0);
        assert_eq!(tuple.is_point(), false);
        assert_eq!(tuple.is_vector(), true);
    }

    #[test]
    fn new_point_is_the_same_as_tuple_with_1_for_w() {
        let point = Tuple::new_point(4.3, -4.2, 3.1);
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(point, tuple);
    }

    #[test]
    fn new_vector_is_the_same_as_tuple_with_0_for_w() {
        let vector = Tuple::new_vector(4.3, -4.2, 3.1);
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(vector, tuple);
    }

    #[test]
    fn adding_two_tuples() {
        let tuple1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let result = tuple1 + tuple2;

        assert_eq!(result, Tuple::new(1.0, 1.0, 6.0, 1.0))
    }

    #[test]
    fn subtracting_two_points() {
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point2 = Tuple::new_point(5.0, 6.0, 7.0);

        let result = point1 - point2;

        assert_eq!(result, Tuple::new_vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Tuple::new_point(3.0, 2.0, 1.0);
        let vector = Tuple::new_vector(5.0, 6.0, 7.0);

        let result = point - vector;

        assert_eq!(result, Tuple::new_point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_two_vectors() {
        let point1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let point2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let result = point1 - point2;

        assert_eq!(result, Tuple::new_vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn negation() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-tuple, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn scalar_multiplication() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = tuple * 3.5;

        assert_eq!(result, Tuple::new(3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    fn scalar_division() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = tuple / 2.0;

        assert_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0))
    }
}
