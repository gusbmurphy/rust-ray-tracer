use ray_tracer::parse::parse_scene_from_yaml;
use ray_tracer::render::create_ppm_from_canvas;
use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).expect("No file path given");
    let target = std::env::args().nth(2).expect("No target name given");

    render_from_yaml(&path, &target)?;

    println!("All done! Created a new file: {}.ppm", target);

    Ok(())
}

pub fn render_from_yaml(yaml_path: &str, target_path: &str) -> Result<(), Box<dyn Error>> {
    let (world, camera) = parse_scene_from_yaml(yaml_path)?;

    let ppm = create_ppm_from_canvas(camera.render(world));

    let mut file = File::create(format!("{}.ppm", target_path))?;
    file.write_all(ppm.as_bytes())?;

    Ok(())
}
