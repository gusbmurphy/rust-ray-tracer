pub trait Tuple {
    fn x(&self) -> &f64;
    fn y(&self) -> &f64;
    fn z(&self) -> &f64;
    fn w(&self) -> &f64;

    fn new(x: f64, y: f64, z: f64) -> Self;

    fn magnitude(&self) -> f64 {
        ((self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2)) as f64).sqrt()
    }

    fn to_array(&self) -> [f64; 4] {
        [*self.x(), *self.y(), *self.z(), *self.w()]
    }
}
