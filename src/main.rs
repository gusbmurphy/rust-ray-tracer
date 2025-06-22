use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use indicatif::ProgressBar;
use ray_tracer::parse::parse_scene_from_yaml;
use ray_tracer::render::create_ppm_from_canvas;
use ray_tracer::render::RenderProgressListener;
use std::time::SystemTime;
use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let path: String = get_input_with_prompt("Path to scene file");
    let target: String = get_input_with_prompt("Name for image");

    let start_time = SystemTime::now();

    render_from_file_at_path_to(path, target.clone())?;

    let end_time = SystemTime::now();
    let time_taken = end_time.duration_since(start_time).ok().unwrap();
    println!(
        "All done in {:#?}! Created a new file: {}.ppm",
        time_taken, target
    );

    Ok(())
}

fn get_input_with_prompt(prompt: &'static str) -> String {
    return Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap();
}

fn render_from_file_at_path_to(path: String, target: String) -> Result<(), Box<dyn Error>> {
    let (world, mut camera) = parse_scene_from_yaml(&path)?;
    let listener: ProgressListener<100> = ProgressListener::default();
    camera.subscribe_to_progress(&listener);

    let ppm = create_ppm_from_canvas(camera.render(world));

    let mut file = File::create(format!("{}.ppm", target))?;
    file.write_all(ppm.as_bytes())?;

    Ok(())
}

struct ProgressListener<const L: u64> {
    bar: ProgressBar,
}

impl<const L: u64> Default for ProgressListener<L> {
    fn default() -> Self {
        ProgressListener {
            bar: ProgressBar::new(L),
        }
    }
}

impl<const L: u64> RenderProgressListener for ProgressListener<L> {
    fn on_progress(&self, completion_percentage: f64) {
        let bar_position = (completion_percentage * L as f64).floor() as u64;
        self.bar.set_position(bar_position);
    }
}
