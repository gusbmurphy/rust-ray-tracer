mod bones;
mod example;
mod physical;
mod render;

mod prelude {
    pub use crate::bones::*;
    pub use crate::physical::*;
    pub use crate::render::*;
}

use example::draw_shading_example_ppm;

use crate::example::draw_circle_example_ppm;
use crate::example::draw_clock_example_ppm;
use crate::example::draw_projectile_example_ppm;
use crate::prelude::Transform;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let projectile_result = draw_projectile_example_to_file();
    let clock_result = draw_clock_example_to_file();
    let sphere_result = draw_sphere_example_to_file();
    let shading_result = draw_shading_example_to_file();

    if shading_result.is_ok() {
        if sphere_result.is_ok() {
            if projectile_result.is_ok() {
                return clock_result;
            } else {
                return projectile_result;
            }
        }
    }

    return sphere_result;
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

fn draw_sphere_example_to_file() -> std::io::Result<()> {
    let ppm_data = draw_circle_example_ppm(Some(
        Transform::new_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            * Transform::new_scaling(0.5, 1.0, 1.0),
    ));

    let mut file = File::create("sphere.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}

fn draw_shading_example_to_file() -> std::io::Result<()> {
    let ppm_data = draw_shading_example_ppm(None);

    let mut file = File::create("shading.ppm")?;
    file.write_all(ppm_data.as_bytes())?;

    Ok(())
}
