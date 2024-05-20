use crate::{prelude::{ppm::create_ppm_from_canvas, *}, render::canvas::Canvas};

pub fn draw_clock_example_ppm() -> String {
    let ray_origin = Point::new(0.0, 0.0, 3.0);

    let mut sphere = Sphere::new();
    let sphere_translation = Transformation::new_translation(0.0, 0.0, -1.0);
    sphere.set_transform(sphere_translation);

    const CANVAS_SIZE: u64 = 300;
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    for x in 1..=CANVAS_SIZE {
        for y in 1..=CANVAS_SIZE {
            let ray = Ray::new(ray_origin, Vector::new(x as f32, y as f32, -1.0));

            let intersections = ray.intersections_with(&sphere);

            let color = if intersections.is_some() { RED } else { BLACK };

            canvas.write_pixel(x as usize, y as usize, color);
        }
    }

    return create_ppm_from_canvas(canvas);
}
