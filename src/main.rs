use ray_tracer::parse::parse_scene_from_yaml;
use ray_tracer::render::create_ppm_from_canvas;
use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).expect("No file path given");
    let target = std::env::args().nth(2).expect("No target name given");

    let (world, camera) = parse_scene_from_yaml(&path)?;

    let ppm = create_ppm_from_canvas(camera.render(world));

    let mut file = File::create(format!("{}.ppm", target))?;
    file.write_all(ppm.as_bytes())?;

    println!("All done! Created a new file: {}.ppm", target);

    Ok(())
}
