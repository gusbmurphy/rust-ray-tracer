use crate::vector::Vector;
use std::ops;

use crate::tuple::Tuple;

// TODO: Should probably use the "approx_eq" macro here...
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z, w: 1.0 }
    }

    pub fn new_with_w(x: f32, y: f32, z: f32, w: f32) -> Self {
        Point { x, y, z, w }
    }
}

impl Tuple for Point {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn get_w(&self) -> f32 {
        0.0
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, v: Vector) -> Self::Output {
        Point::new(self.x + v.get_x(), self.y + v.get_y(), self.z + v.get_z())
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, v: Vector) -> Self::Output {
        Point::new(self.x - v.get_x(), self.y - v.get_y(), self.z - v.get_z())
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, p: Point) -> Self::Output {
        Vector::new(self.x - p.get_x(), self.y - p.get_y(), self.z - p.get_z())
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new_with_w(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point::new_with_w(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl ops::Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Point::new_with_w(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    use super::*;

    #[test]
    fn adding_to_vector() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);

        let result = point + vector;

        assert_eq!(result, Point::new_with_w(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        let point1 = Point::new(3.0, 2.0, 1.0);
        let point2 = Point::new(5.0, 6.0, 7.0);

        let result = point1 - point2;

        assert_eq!(result, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);

        let result = point - vector;

        assert_eq!(result, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negation() {
        let point = Point::new_with_w(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-point, Point::new_with_w(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn scalar_multiplication() {
        let point = Point::new_with_w(1.0, -2.0, 3.0, -4.0);

        let result = point * 3.5;

        assert_eq!(result, Point::new_with_w(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn scalar_division() {
        let point = Point::new_with_w(1.0, -2.0, 3.0, -4.0);

        let result = point / 2.0;

        assert_eq!(result, Point::new_with_w(0.5, -1.0, 1.5, -2.0));
    }
}
