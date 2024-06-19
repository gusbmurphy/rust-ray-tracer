pub trait Tuple {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
    fn w(&self) -> &f32;

    fn new(x: f32, y: f32, z: f32) -> Self;

    fn magnitude(&self) -> f32 {
        ((self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2)) as f32).sqrt()
    }

    fn to_array(&self) -> [f32; 4] {
        [*self.x(), *self.y(), *self.z(), *self.w()]
    }
}
