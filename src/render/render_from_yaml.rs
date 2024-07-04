use crate::parse::parse_scene_from_yaml;
use std::{error::Error, fs::File, io::Write};
use crate::prelude::*;

pub fn render_from_yaml(yaml_path: &str, target_path: &str) -> Result<(), Box<dyn Error>> {
    let (world, camera) = parse_scene_from_yaml(yaml_path)?;

    let ppm = create_ppm_from_canvas(camera.render(world));

    let mut file = File::create(format!("{}.ppm", target_path))?;
    file.write_all(ppm.as_bytes())?;

    Ok(())
}
