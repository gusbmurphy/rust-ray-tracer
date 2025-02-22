use crate::ui::app::ShapeInfo;
use egui::Ui;

pub fn shape_menu(ui: &mut Ui, info: &mut ShapeInfo) {
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
            ui.add(egui::Slider::new(&mut info.transparency_percentage, 0.0..=100.0).suffix("%"));
            ui.end_row();

            ui.label("Position");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut info.position.x).speed(0.1));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut info.position.y).speed(0.1));
                ui.label("Z:");
                ui.add(egui::DragValue::new(&mut info.position.z).speed(0.1));
            });
            ui.end_row();

            ui.label("Rotation");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut info.rot_x).speed(0.1));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut info.rot_y).speed(0.1));
                ui.label("Z:");
                ui.add(egui::DragValue::new(&mut info.rot_z).speed(0.1));
            });
            ui.end_row();
        });
}
