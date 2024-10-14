use ray_tracer::ui::SceneBuilder;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Trace Rayer",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(SceneBuilder::new(cc)))
        }),
    )
}
