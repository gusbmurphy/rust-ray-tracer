use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use ray_tracer::parse::parse_scene_from_yaml;
use ray_tracer::render::create_ppm_from_canvas;
use ray_tracer::render::RenderProgressListener;
use std::time::SystemTime;
use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to scene file")
        .interact_text()
        .unwrap();

    let target: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name for image")
        .interact_text()
        .unwrap();

    let start_time = SystemTime::now();

    let (world, mut camera) = parse_scene_from_yaml(&path)?;
    let listener = ProgressListener::default();
    camera.subscribe_to_progress(&listener);

    let ppm = create_ppm_from_canvas(camera.render(world));

    let mut file = File::create(format!("{}.ppm", target))?;
    file.write_all(ppm.as_bytes())?;

    let end_time = SystemTime::now();
    let time_taken = end_time.duration_since(start_time).ok().unwrap();
    println!(
        "All done in {:#?}! Created a new file: {}.ppm",
        time_taken, target
    );

    Ok(())
}

#[derive(Default)]
struct ProgressListener;

impl RenderProgressListener for ProgressListener {
    fn on_progress(&self, completion: f64) {
        let completion_percentage = 100f64 * completion;

        if completion_percentage < 100f64 {
            print!("\r{:.2}% complete... ", completion_percentage)
        }
    }
}
