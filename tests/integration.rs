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
    let (world, camera) =
        parse_scene_from_yaml("tests/scenes/scene_with_plane_and_sphere.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn three_spheres_on_a_plane() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/3_spheres_2_planes.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn sphere_with_stripes() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/sphere_with_stripes.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn sphere_with_stripes_and_transforms() {
    let (world, camera) =
        parse_scene_from_yaml("tests/scenes/sphere_with_stripes_and_transforms.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn sphere_with_every_transform() {
    let (world, camera) =
        parse_scene_from_yaml("tests/scenes/sphere_with_every_transform.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn sphere_with_gradient() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/sphere_with_gradient.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn sphere_with_rings() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/sphere_with_rings.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn sphere_with_3d_checkers() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/sphere_with_checkers.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn sphere_with_blended_pattern() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/blended_patterns.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn small_reflective_scene() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/reflective_floor.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}

#[test]
fn transparent_sphere() {
    let (world, camera) = parse_scene_from_yaml("tests/scenes/transparent_sphere.yaml").unwrap();

    let ppm = create_ppm_from_canvas(camera.render(world));

    insta::assert_yaml_snapshot!(ppm);
}
