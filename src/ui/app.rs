use crate::prelude::*;
use crate::ui::shape_menu::shape_menu;
use eframe::App;
use egui::Color32;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use egui::Ui;
use std::rc::Rc;

use crate::render::Color;

pub struct SceneBuilder {
    shapes: Vec<ShapeInfo>,
    camera: CameraInfo,
    image_texture: Option<TextureHandle>,
}

struct Position {
    x: f64,
    y: f64,
    z: f64,
}

struct CameraInfo {
    horizontal_size: u32,
    vertical_size: u32,
    field_of_view: f64,
    pub position: Position,
    pub target: Position,
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            horizontal_size: 100,
            vertical_size: 100,
            field_of_view: 100.0,
            position: Position {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            target: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

pub struct ShapeInfo {
    pub name: String,
    pub shape_type: ShapeType,
    pub color: [f32; 3],
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub refractive_index: f64,
    pub transparency_percentage: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
}

impl Default for ShapeInfo {
    fn default() -> Self {
        Self {
            name: "Sphere 1".to_string(),
            shape_type: ShapeType::Sphere,
            color: [0.1, 0.1, 0.1],
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            refractive_index: 1.0,
            transparency_percentage: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
        }
    }
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            shapes: vec![ShapeInfo::default()],
            camera: CameraInfo::default(),
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
            ui.collapsing("Camera", |ui| {
                egui::Grid::new("camera-grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Position");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut self.camera.position.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut self.camera.position.y).speed(0.1));
                            ui.label("Z:");
                            ui.add(egui::DragValue::new(&mut self.camera.position.z).speed(0.1));
                        });
                        ui.end_row();

                        // TODO: Feels like there could be a better label than "To"
                        ui.label("To");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut self.camera.target.x).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut self.camera.target.y).speed(0.1));
                            ui.label("Z:");
                            ui.add(egui::DragValue::new(&mut self.camera.target.z).speed(0.1));
                        });
                        ui.end_row();
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Add shape: ");

                // TODO: Couldn't this string come from the `ShapeType`?
                self.shape_button(ui, "Sphere", ShapeType::Sphere);
                self.shape_button(ui, "Plane", ShapeType::Plane);
            });

            for info in &mut self.shapes {
                ui.collapsing(info.name.clone(), |ui| {
                    shape_menu(ui, info);
                });
            }

            if let Some(texture) = &self.image_texture {
                ui.image((texture.id(), texture.size_vec2()));
            };
        });
    }
}

impl SceneBuilder {
    fn shape_button(&mut self, ui: &mut Ui, description: &str, shape_type: ShapeType) {
        if ui.button(description).clicked() {
            let mut new_info = ShapeInfo::default();
            new_info.shape_type = shape_type;
            new_info.name =
                description.to_string() + " " + (self.shapes.len() + 1).to_string().as_str();
            self.shapes.push(new_info);
        }
    }

    fn build_image(&mut self, ctx: &Context) {
        let mut world = World::new();

        for info in &self.shapes {
            let material = MaterialBuilder::new()
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
                .transparency(info.transparency_percentage / 100.0)
                .build();

            let transform = Transform::translation(info.pos_x, info.pos_y, info.pos_z)
                * Transform::x_rotation(info.rot_x)
                * Transform::y_rotation(info.rot_y)
                * Transform::z_rotation(info.rot_z);

            let shape: Rc<dyn Shape> = match info.shape_type {
                ShapeType::Plane => {
                    let mut plane = Plane::new_with_material(material);
                    plane.set_transform(transform);
                    Rc::new(plane)
                }
                ShapeType::Sphere => {
                    let mut sphere = Sphere::new_with_material(material);
                    sphere.set_transform(transform);
                    Rc::new(sphere)
                }
            };

            world.add_shape(shape);
        }

        let camera_transform = Transform::view(
            Point::new(
                self.camera.position.x,
                self.camera.position.y,
                self.camera.position.z,
            ),
            Point::new(
                self.camera.target.x,
                self.camera.target.y,
                self.camera.target.z,
            ),
            POSITIVE_Y,
        );

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
                let inverted_x = (image_width - 1) as usize - x;
                let inverted_y = (image_height - 1) as usize - y;

                let rgb_values = canvas.pixel_at(inverted_x, inverted_y).to_rgb();
                let color = Color32::from_rgb(rgb_values[0], rgb_values[1], rgb_values[2]);

                let pixel_index = x + (y * image_width as usize);
                image.pixels[pixel_index] = color;
            }
        }

        self.image_texture = Some(ctx.load_texture("result_image", image, Default::default()));
    }
}
