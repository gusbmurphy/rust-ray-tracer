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

use prelude::*;
use projectile_example::draw_projectile_example_to_file;
use clock_example::draw_clock_example_to_file;

fn main() -> Result<(), std::io::Error> {
    let projectile_result = draw_projectile_example_to_file();
    let clock_result = draw_clock_example_to_file(100, 25);

    if projectile_result.is_ok() {
        return clock_result;
    } else {
        return projectile_result;
    }
}
