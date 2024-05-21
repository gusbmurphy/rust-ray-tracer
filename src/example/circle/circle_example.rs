use crate::{
    prelude::{ppm::create_ppm_from_canvas, *},
    render::canvas::Canvas,
};

pub fn draw_circle_example_ppm() -> String {
    let ray_origin = Point::new(0.0, 0.0, 3.0);

    let sphere = Sphere::new();

    const WALL_Z: f32 = -6.0;
    const WALL_WIDTH: f32 = 6.0;

    const CANVAS_SIZE: u64 = 300;
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    for canvas_x in 0..CANVAS_SIZE {
        let wall_x: f32 = (canvas_x as f32 / CANVAS_SIZE as f32) * WALL_WIDTH - (WALL_WIDTH * 0.5);

        for canvas_y in 0..CANVAS_SIZE {
            let wall_y: f32 = (canvas_y as f32 / CANVAS_SIZE as f32) * WALL_WIDTH - (WALL_WIDTH * 0.5);

            let ray = Ray::new(ray_origin, Vector::new(wall_x, wall_y, WALL_Z));

            let intersections = ray.intersections_with(&sphere);

            let color = if intersections.is_some() { RED } else { BLACK };

            canvas.write_pixel(canvas_x as usize, canvas_y as usize, color);
        }
    }

    return create_ppm_from_canvas(canvas);
}
