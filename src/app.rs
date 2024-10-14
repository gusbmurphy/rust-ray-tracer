use crate::prelude::Camera;
use crate::prelude::Point;
use crate::prelude::Shape;
use crate::prelude::Transform;
use crate::prelude::Tuple;
use crate::prelude::ORIGIN;
use crate::prelude::POSITIVE_Y;
use eframe::App;
use egui::emath::Numeric;
use egui::Color32;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;

use crate::prelude::MaterialBuilder;
use crate::prelude::Sphere;
use crate::prelude::World;
use crate::render::Color;

pub struct SceneBuilder {
    sphere_infos: Vec<SphereInfo>,
    image_texture: Option<TextureHandle>,
}

struct SphereInfo {
    name: String,
    color: [f32; 3],
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    refractive_index: f64,
    transparency: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Default for SphereInfo {
    fn default() -> Self {
        Self {
            name: "Sphere 1".to_string(),
            color: [0.1, 0.1, 0.1],
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            refractive_index: 1.0,
            transparency: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            sphere_infos: vec![SphereInfo::default()],
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
                    self.build_image(ctx);
                };
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scene Builder");

            for info in &mut self.sphere_infos {
                ui.label(info.name.to_owned());

                egui::Grid::new(info.name.to_owned() + "-grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Color");
                        ui.color_edit_button_rgb(&mut info.color);
                        ui.end_row();

                        ui.label("Ambient");
                        ui.add(egui::DragValue::new(&mut info.ambient).speed(0.1));
                        ui.end_row();

                        ui.label("Diffuse");
                        ui.add(egui::DragValue::new(&mut info.diffuse).speed(0.1));
                        ui.end_row();

                        ui.label("Specular");
                        ui.add(egui::DragValue::new(&mut info.specular).speed(0.1));
                        ui.end_row();

                        ui.label("Shininess");
                        ui.add(egui::DragValue::new(&mut info.shininess).speed(0.1));
                        ui.end_row();

                        ui.label("Reflective");
                        ui.add(egui::DragValue::new(&mut info.reflective).speed(0.1));
                        ui.end_row();

                        ui.label("Refractive index");
                        ui.add(egui::DragValue::new(&mut info.refractive_index).speed(0.1));
                        ui.end_row();

                        ui.label("Transparency");
                        ui.add(egui::DragValue::new(&mut info.transparency).speed(0.1));
                        ui.end_row();

                        ui.label("Position");
                        ui.add(egui::DragValue::new(&mut info.x).speed(0.1));
                        ui.add(egui::DragValue::new(&mut info.y).speed(0.1));
                        ui.add(egui::DragValue::new(&mut info.z).speed(0.1));
                        ui.end_row();
                    });
            }

            if ui.button("Add sphere").clicked() {
                let mut new_info = SphereInfo::default();
                new_info.name =
                    "Sphere ".to_string() + (self.sphere_infos.len() + 1).to_string().as_str();
                self.sphere_infos.push(new_info);
            }

            if let Some(texture) = &self.image_texture {
                ui.image((texture.id(), texture.size_vec2()));
            };
        });
    }
}

impl SceneBuilder {
    fn build_image(&mut self, ctx: &Context) {
        let mut world = World::new();

        for info in &self.sphere_infos {
            let mut sphere = Sphere::new_with_material(
                MaterialBuilder::new()
                    .flat_color(Color::new(
                        info.color[0].to_f64(),
                        info.color[1].to_f64(),
                        info.color[2].to_f64(),
                    ))
                    .ambient(info.ambient)
                    .diffuse(info.diffuse)
                    .specular(info.specular)
                    .shininess(info.shininess)
                    .reflective(info.reflective)
                    .refractive_index(info.refractive_index)
                    .transparency(info.transparency)
                    .build(),
            );
            sphere.set_transform(Transform::translation(info.x, info.y, info.z));
            world.add_sphere(sphere);
        }

        let camera_transform = Transform::view(Point::new(0.0, 0.0, -5.0), ORIGIN, POSITIVE_Y);

        let image_height = 100;
        let image_width = 100;
        let camera = Camera::new_with_transform(image_height, image_width, 100.0, camera_transform);
        let canvas = camera.render(world);

        let mut image = ColorImage::new(
            [image_height as usize, image_width as usize],
            Color32::BLACK,
        );

        for y in 0..image_height as usize {
            for x in 0..image_width as usize {
                let rgb_values = canvas.pixel_at(x, y).to_rgb();
                let color = Color32::from_rgb(rgb_values[0], rgb_values[1], rgb_values[2]);

                let pixel_index = x + (y * image_width as usize);
                image.pixels[pixel_index] = color;
            }
        }

        self.image_texture = Some(ctx.load_texture("result_image", image, Default::default()));
    }
}
