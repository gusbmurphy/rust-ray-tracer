use crate::prelude::*;

use crate::render::*;

pub fn draw_shading_example_ppm(transform: Option<Transform>) -> String {
    let ray_origin = Point::new(0.0, 0.0, 3.0);

    let mut sphere = Sphere::new();

    if let Some(t) = transform {
        sphere.set_transform(t);
    }

    const WALL_Z: f32 = -6.0;
    const WALL_WIDTH: f32 = 6.0;

    const CANVAS_SIZE: u64 = 300;
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 0.2, 1.0);
    let light = PointLight::new(light_color, light_position);

    for canvas_x in 0..CANVAS_SIZE {
        let wall_x: f32 = (canvas_x as f32 / CANVAS_SIZE as f32) * WALL_WIDTH - (WALL_WIDTH * 0.5);

        for canvas_y in 0..CANVAS_SIZE {
            let wall_y: f32 =
                (canvas_y as f32 / CANVAS_SIZE as f32) * WALL_WIDTH - (WALL_WIDTH * 0.5);

            let ray = Ray::new(ray_origin, Vector::new(wall_x, wall_y, WALL_Z).normalize());

            let intersections = ray.intersections_with(&sphere);

            let color = if intersections.is_some() {
                let hit_t = intersections.unwrap()[0];
                let hit_point = ray.get_position(*hit_t.get_t());

                let lighting_calculator = LightingCalculator::new(
                    -ray.get_direction().to_owned(),
                    sphere.normal_at(hit_point),
                    light,
                );

                lighting_calculator.get_color_for_material_at(
                    sphere.get_material().to_owned(),
                    hit_point,
                    false,
                )
            } else {
                BLACK
            };

            canvas.write_pixel(canvas_x as usize, canvas_y as usize, color);
        }
    }

    return create_ppm_from_canvas(canvas);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snapshot() {
        let result = draw_shading_example_ppm(None);
        insta::assert_yaml_snapshot!(result);
    }
}
