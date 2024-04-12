#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Intersection<'a, T> {
    time: f32,
    object: &'a T,
}

impl<'a, T> Intersection<'a, T>
where
    T: Intersectable,
{
    pub fn new(time: f32, object: &'a T) -> Self {
        Intersection { time, object }
    }

    pub fn get_intersected(self) -> &'a T {
        self.object
    }

    pub fn get_t(&self) -> f32 {
        self.time
    }
}

pub trait Intersectable {}
