pub const EPSILON: f64 = 0.00001;

pub fn close_enough(a: &f64, b: &f64) -> bool {
    (a - b).abs() < EPSILON
}
