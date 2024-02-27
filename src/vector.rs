use std::ops;

use crate::{close_enough::close_enough, point::Point, tuple::Tuple};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.get_magnitude();

        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
}

impl Tuple for Vector {
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

impl ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, p: Point) -> Self::Output {
        Point::new(self.x + p.get_x(), self.y + p.get_y(), self.z + p.get_z())
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl PartialEq<Vector> for Vector {
    fn eq(&self, other: &Vector) -> bool {
        close_enough(&self.x, &other.x)
            && close_enough(&self.y, &other.y)
            && close_enough(&self.z, &other.z)
    }
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    let x = a.y * b.z - a.z * b.y;
    let y = a.z * b.x - a.x * b.z;
    let z = a.x * b.y - a.y * b.x;

    Vector::new(x, y, z)
}

pub fn dot(a: &Vector, b: &Vector) -> f32 {
    a.get_x() * b.get_x() + a.get_y() * b.get_y() + a.get_z() * b.get_z()
}

#[cfg(test)]
mod test {
    use crate::close_enough::close_enough;
    use crate::point::Point;

    use super::*;

    #[test]
    fn cross_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        let result1 = cross(&a, &b);
        let result2 = cross(&b, &a);

        assert_eq!(result1, Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(result2, Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn normalized() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let normalized_vector = vector.normalize();

        assert_eq!(
            normalized_vector,
            Vector::new(
                1.0 / (14.0f32).sqrt(),
                2.0 / (14.0f32).sqrt(),
                3.0 / (14.0f32).sqrt()
            )
        );

        assert!(close_enough(&normalized_vector.get_magnitude(), &1.0));
    }

    #[test]
    fn vector_magnitude() {
        let vector = Vector::new(-1.0, -2.0, -3.0);

        let result = vector.get_magnitude();

        assert_eq!(result, (14.0 as f32).sqrt());
    }

    #[test]
    fn adding_point() {
        let vector = Vector::new(-2.0, 3.0, 1.0);
        let point = Point::new(3.0, -2.0, 5.0);

        let result = vector + point;

        assert_eq!(result, Point::new_with_w(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn adding_vectors() {
        let vector1 = Vector::new(-5.0, 3.0, 1.0);
        let vector2 = Vector::new(2.0, -2.0, -32.0);

        let result = vector1 + vector2;

        assert_eq!(result, Vector::new(-3.0, 1.0, -31.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let vector1 = Vector::new(3.0, 2.0, 1.0);
        let vector2 = Vector::new(5.0, 6.0, 7.0);

        let result = vector1 - vector2;

        assert_eq!(result, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negation() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-vector, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn scalar_multiplication() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let result = vector * 3.5;

        assert_eq!(result, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn scalar_division() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let result = vector / 2.0;

        assert_eq!(result, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn dot_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        let result = dot(&a, &b);

        assert_eq!(result, 20.0);
    }
}
