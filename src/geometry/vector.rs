use std::ops;

use super::tuple::Tuple;
use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Vector::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    pub fn reflect_around(&self, reflection_vector: &Vector) -> Vector {
        self.to_owned() - reflection_vector.to_owned() * 2.0 * dot(self, reflection_vector)
    }
}

impl Tuple for Vector {
    fn x(&self) -> &f64 {
        &self.x
    }

    fn y(&self) -> &f64 {
        &self.y
    }

    fn z(&self) -> &f64 {
        &self.z
    }

    fn w(&self) -> &f64 {
        &0.0
    }

    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }
}

impl ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, p: Point) -> Self::Output {
        Point::new(self.x + p.x(), self.y + p.y(), self.z + p.z())
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

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
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

// TODO: What if "cross" and "dot" lived on Vector?
pub fn cross(a: &Vector, b: &Vector) -> Vector {
    let x = a.y * b.z - a.z * b.y;
    let y = a.z * b.x - a.x * b.z;
    let z = a.x * b.y - a.y * b.x;

    Vector::new(x, y, z)
}

pub fn dot(a: &Vector, b: &Vector) -> f64 {
    a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
}

pub const POSITIVE_X: Vector = Vector {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub const NEGATIVE_X: Vector = Vector {
    x: -1.0,
    y: 0.0,
    z: 0.0,
};

pub const POSITIVE_Y: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub const NEGATIVE_Y: Vector = Vector {
    x: 0.0,
    y: -1.0,
    z: 0.0,
};

pub const POSITIVE_Z: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

pub const NEGATIVE_Z: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};

#[cfg(test)]
mod test {
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
                1.0 / (14.0f64).sqrt(),
                2.0 / (14.0f64).sqrt(),
                3.0 / (14.0f64).sqrt()
            )
        );

        assert!(close_enough(&normalized_vector.magnitude(), &1.0));
    }

    #[test]
    fn vector_magnitude() {
        let vector = Vector::new(-1.0, -2.0, -3.0);

        let result = vector.magnitude();

        assert_eq!(result, (14.0 as f64).sqrt());
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

    #[test]
    fn reflecting_a_vector_at_45_degrees() {
        let vector = Vector::new(1.0, -1.0, 0.0);
        let normal = Vector::new(0.0, 1.0, 0.0);

        let reflection = vector.reflect_around(&normal);

        assert_eq!(reflection, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_a_slanted_surface() {
        let vector = Vector::new(0.0, -1.0, 0.0);
        let normal = Vector::new(2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0);

        let reflection = vector.reflect_around(&normal);

        assert_eq!(reflection, Vector::new(1.0, 0.0, 0.0));
    }
}
