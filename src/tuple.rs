pub trait Tuple {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
    fn get_w(&self) -> f64;

    fn get_magnitude(&self) -> f64 {
        ((self.get_x().powi(2)
            + self.get_y().powi(2)
            + self.get_z().powi(2)
            + self.get_w().powi(2)) as f64)
            .sqrt()
    }
}
