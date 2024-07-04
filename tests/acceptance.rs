use ray_tracer::parse::parse_scene_from_yaml;
use ray_tracer::render::create_ppm_from_canvas;

#[test]
fn just_three_spheres() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/three_spheres.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn plane_and_sphere() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/scene_with_plane_and_sphere.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}
