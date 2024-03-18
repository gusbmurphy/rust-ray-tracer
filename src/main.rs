mod bones;
mod example;
mod render;

mod prelude {
    pub use crate::bones::*;
}

use crate::example::clock_example::draw_clock_example_to_file;
use crate::example::projectile_example::draw_projectile_example_to_file;

fn main() -> Result<(), std::io::Error> {
    let projectile_result = draw_projectile_example_to_file();
    let clock_result = draw_clock_example_to_file(100, 25);

    if projectile_result.is_ok() {
        return clock_result;
    } else {
        return projectile_result;
    }
}
