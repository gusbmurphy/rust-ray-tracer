use crate::prelude::Camera;
use crate::prelude::Point;
use crate::prelude::Transform;
use crate::prelude::Tuple;
use crate::prelude::ORIGIN;
use crate::prelude::POSITIVE_Y;
use eframe::App;
use egui::emath::Numeric;
use egui::Color32;
use egui::ColorImage;
use egui::TextureHandle;

use crate::prelude::MaterialBuilder;
use crate::prelude::Sphere;
use crate::prelude::World;
use crate::render::Color;

pub struct SceneBuilder {
    color: [f32; 3],
    image_texture: Option<TextureHandle>,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            color: [0.1, 0.1, 0.1],
            image_texture: None,
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

                    let image_height = 100;
                    let image_width = 100;
                    let camera = Camera::new_with_transform(
                        image_height,
                        image_width,
                        100.0,
                        camera_transform,
                    );
                    let canvas = camera.render(world);

                    let mut image = ColorImage::new(
                        [image_height as usize, image_width as usize],
                        Color32::BLACK,
                    );

                    for y in 0..image_height as usize {
                        for x in 0..image_width as usize {
                            let rgb_values = canvas.pixel_at(x, y).to_rgb();
                            let color =
                                Color32::from_rgb(rgb_values[0], rgb_values[1], rgb_values[2]);

                            let pixel_index = x + (y * image_width as usize);
                            image.pixels[pixel_index] = color;
                        }
                    }

                    self.image_texture =
                        Some(ctx.load_texture("result_image", image, Default::default()));
                };
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scene Builder");

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Color");
                    ui.color_edit_button_rgb(&mut self.color);
                    ui.end_row();
                });

            if let Some(texture) = &self.image_texture {
                ui.image((texture.id(), texture.size_vec2()));
            };
        });
    }
}
