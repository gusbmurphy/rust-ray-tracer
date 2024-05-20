mod bones;
mod example;
mod render;

mod prelude {
    pub use crate::bones::*;
    pub use crate::render::*;
}

use crate::example::draw_clock_example_ppm;
use crate::example::draw_projectile_example_ppm;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let projectile_result = draw_projectile_example_to_file();
    let clock_result = draw_clock_example_to_file();

    if projectile_result.is_ok() {
        return clock_result;
    } else {
        return projectile_result;
    }
}

fn draw_clock_example_to_file() -> std::io::Result<()> {
    let ppm_data = draw_clock_example_ppm(100, 25);

    let mut file = File::create("clock.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}

fn draw_projectile_example_to_file() -> std::io::Result<()> {
    let ppm_data = draw_projectile_example_ppm();

    let mut file = File::create("projectile.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
