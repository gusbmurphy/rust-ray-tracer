pub struct Intersection {
    time: f32,
    object: Box<dyn Intersectable>,
}

pub trait Intersectable {}
