use crate::prelude::Camera;
use crate::prelude::Point;
use crate::prelude::Transform;
use crate::prelude::Tuple;
use crate::prelude::ORIGIN;
use crate::prelude::POSITIVE_Y;
use crate::render::create_ppm_from_canvas;
use eframe::App;
use egui::emath::Numeric;
use egui::UiBuilder;
use std::fs::File;
use std::io::Write;

use crate::prelude::MaterialBuilder;
use crate::prelude::Sphere;
use crate::prelude::World;
use crate::render::Color;

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

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Build").clicked() {
                    let mut world = World::new();
                    let sphere = Sphere::new_with_material(
                        MaterialBuilder::new()
                            .flat_color(Color::new(
                                self.color[0].to_f64(),
                                self.color[1].to_f64(),
                                self.color[2].to_f64(),
                            ))
                            .build(),
                    );
                    world.add_sphere(sphere);

                    let camera_transform =
                        Transform::view(Point::new(0.0, 0.0, -5.0), ORIGIN, POSITIVE_Y);
                    let camera = Camera::new_with_transform(100, 100, 100.0, camera_transform);

                    let ppm = create_ppm_from_canvas(camera.render(world));
                    let mut file = File::create("output/trying.ppm").unwrap();
                    let _ = file.write_all(ppm.as_bytes());
                };
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
