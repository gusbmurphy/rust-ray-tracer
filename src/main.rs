mod close_enough;
mod color;
mod matrix;
mod point;
mod render;
mod example;
mod transformation;
mod tuple;
mod vector;

mod prelude {
    pub use crate::point::*;
    pub use crate::tuple::*;
}

use crate::example::clock_example::draw_clock_example_to_file;
use crate::example::projectile_example::draw_projectile_example_to_file;
use prelude::*;

fn main() -> Result<(), std::io::Error> {
    let projectile_result = draw_projectile_example_to_file();
    let clock_result = draw_clock_example_to_file(100, 25);

    if projectile_result.is_ok() {
        return clock_result;
    } else {
        return projectile_result;
    }
}
