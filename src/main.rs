use ray_tracer::parse::parse_scene_from_yaml;
use ray_tracer::render::create_ppm_from_canvas;
use ray_tracer::render::RenderProgressListener;
use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).expect("No file path given");
    let target = std::env::args().nth(2).expect("No target name given");

    let (world, mut camera) = parse_scene_from_yaml(&path)?;
    let listener = ProgressListener::default();
    camera.subscribe_to_progress(&listener);

    let ppm = create_ppm_from_canvas(camera.render(world));

    let mut file = File::create(format!("{}.ppm", target))?;
    file.write_all(ppm.as_bytes())?;

    println!("All done! Created a new file: {}.ppm", target);

    Ok(())
}

#[derive(Default)]
struct ProgressListener;

impl RenderProgressListener for ProgressListener {
    fn on_progress(&self, completion: f64) {
        println!("{}% complete...", completion)
    }
}
