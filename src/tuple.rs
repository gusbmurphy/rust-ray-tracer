pub trait Tuple {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_z(&self) -> f32;
    fn get_w(&self) -> f32;

    fn get_magnitude(&self) -> f32 {
        ((self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2) + self.get_w().powi(2))
            as f32)
            .sqrt()
    }
}
