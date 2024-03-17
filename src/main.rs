mod canvas;
mod clock_example;
mod close_enough;
mod color;
mod matrix;
mod point;
mod ppm;
mod projectile_example;
mod projectile_tick;
mod transformation;
mod tuple;
mod vector;

mod prelude {
    pub use crate::point::*;
    pub use crate::tuple::*;
}

use clock_example::draw_clock_example;
use prelude::*;
use projectile_example::draw_projectile_example;

fn main() {
    draw_projectile_example();
    draw_clock_example(1000, 100);
}
