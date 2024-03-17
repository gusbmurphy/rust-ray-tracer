mod canvas;
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

use prelude::*;
use projectile_example::draw_projectile_example;

fn main() -> std::io::Result<()> {
    draw_projectile_example()
}
