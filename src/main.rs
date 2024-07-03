mod example;
mod geometry;
mod parse;
mod physical;
mod render;

mod prelude {
    pub use crate::geometry::*;
    pub use crate::physical::*;
    pub use crate::render::*;
}

use example::draw_scene_ppm;

use crate::example::draw_clock_example_ppm;
use crate::example::draw_projectile_example_ppm;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let projectile_result = draw_projectile_example_to_file();
    let clock_result = draw_clock_example_to_file();
    let scene_result = draw_scene_example_to_file();

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

fn draw_scene_example_to_file() -> std::io::Result<()> {
    let ppm_data = draw_scene_ppm(1000, 500);

    let mut file = File::create("scene.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
