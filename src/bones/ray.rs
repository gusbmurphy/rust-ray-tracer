use crate::prelude::*;

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn get_position(&self, time: f32) -> Point {
        self.origin + self.direction * time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn getting_positions_on_ray() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(ray.get_position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.get_position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.get_position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.get_position(2.5), Point::new(4.5, 3.0, 4.0));
    }
}
