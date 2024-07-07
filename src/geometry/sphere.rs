use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f64,
    transform: Transform,
    material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Transform::new(IDENTITY_MATRIX),
            material: Material::new(),
        }
    }

    pub fn center(&self) -> &Point {
        &self.center
    }
}

impl Shape for Sphere {
    fn normal_at(&self, world_space_point: Point) -> Vector {
        let transform_inverse = self.transform.invert().unwrap();
        let object_space_point = transform_inverse * world_space_point;

        let object_space_normal = object_space_point - Point::new(0.0, 0.0, 0.0);

        let world_space_normal = transform_inverse.matrix().transpose() * object_space_normal;

        return world_space_normal.normalize();
    }

    fn times_of_intersections_with<'s, 'r>(&'s self, ray: &'r Ray) -> Vec<f64>
    where
        'r: 's,
    {
        let ray_in_object_space = self.transform().invert().unwrap() * ray;
        let vector_from_sphere_to_ray = *ray_in_object_space.origin() - *self.center();

        let a = dot(
            ray_in_object_space.direction(),
            ray_in_object_space.direction(),
        );
        let b = 2f64 * dot(ray_in_object_space.direction(), &vector_from_sphere_to_ray);
        let c = dot(&vector_from_sphere_to_ray, &vector_from_sphere_to_ray) - 1f64;

        let discriminant = b.powi(2) - 4f64 * a * c;

        if discriminant < 0f64 {
            return Vec::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2f64 * a);
        let t2 = (-b + discriminant.sqrt()) / (2f64 * a);

        return vec![t1, t2];
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transformation: Transform) {
        self.transform = transformation;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn shape_type(&self) -> ShapeType {
        ShapeType::Sphere
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn default_sphere_transform() {
        let sphere = Sphere::new();
        assert_eq!(sphere.transform().to_owned(), IDENTITY_MATRIX);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let sphere = Sphere::new();
        assert_eq!(sphere.material, Material::new())
    }

    #[test]
    fn changing_sphere_transform() {
        let mut sphere = Sphere::new();
        let translation = Transform::translation(2.0, 2.0, 4.0);

        sphere.set_transform(translation);

        assert!(translation == sphere.transform().to_owned());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::scaling(2.0, 2.0, 2.0));

        let times = sphere.times_of_intersections_with(&ray);

        assert_eq!(times.len(), 2);

        assert!(times.iter().any(|t| *t == 3.0));
        assert!(times.iter().any(|t| *t == 7.0));
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::translation(5.0, 0.0, 0.0));

        let times = sphere.times_of_intersections_with(&ray);

        assert!(times.is_empty());
    }

    #[test]
    fn getting_normal_on_x_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(normal, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn getting_normal_on_y_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(0.0, 1.0, 0.0));

        assert_eq!(normal, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn getting_normal_on_z_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(normal, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn getting_normal_at_a_nonaxial_point() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));

        assert_eq!(
            normal,
            Vector::new(
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0,
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(normal, normal.normalize());
    }

    #[test]
    fn computing_normal_on_a_translated_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::translation(0.0, 1.0, 0.0));

        let normal = sphere.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(normal, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_a_scaled_and_rotated_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transform(Transform::scaling(1.0, 0.5, 1.0) * Transform::z_rotation(PI / 5.0));

        let normal = sphere.normal_at(Point::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));

        assert_eq!(normal, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn intersection_through_middle_of_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let times = sphere.times_of_intersections_with(&ray);
        assert_eq!(times.len(), 2);

        assert!(times.iter().any(|t| *t == 4.0));
        assert!(times.iter().any(|t| *t == 6.0));
    }

    #[test]
    fn ray_missing_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let times = sphere.times_of_intersections_with(&ray);

        assert!(times.is_empty());
    }

    #[test]
    fn ray_originating_inside_of_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let times = sphere.times_of_intersections_with(&ray);
        assert_eq!(times.len(), 2);

        assert!(times.iter().any(|t| *t == -1.0));
        assert!(times.iter().any(|t| *t == 1.0));
    }

    #[test]
    fn ray_ahead_of_a_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();

        let times = sphere.times_of_intersections_with(&ray);
        assert_eq!(times.len(), 2);

        assert!(times.iter().any(|t| *t == -6.0));
        assert!(times.iter().any(|t| *t == -4.0));
    }
}
