use eframe::App;
use egui::DragValue;
use egui::UiBuilder;

pub struct SceneBuilder {
    color: [f32; 3],
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            color: [0.1, 0.1, 0.1],
        }
    }
}

impl SceneBuilder {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl App for SceneBuilder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Scene Builder");

            ui.scope_builder(UiBuilder::new(), |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Color");
                        ui.color_edit_button_rgb(&mut self.color);
                        ui.end_row();
                    });
            });
        });
    }
}
