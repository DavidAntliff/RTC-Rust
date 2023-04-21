// Chapter 9: Planes

use crate::intersections::Intersections;
use crate::materials::Material;
use crate::matrices::{inverse, transpose, Matrix4};
use crate::planes::Plane;
use crate::rays::Ray;
use crate::spheres::Sphere;
use crate::tuples::{normalize, Point, Vector};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Shape {
    pub shape: ShapeEnum,
    transform: Matrix4,
    inverse_transform: Matrix4,
    pub material: Material,
}

impl Shape {
    pub fn sphere(id: i32) -> Shape {
        Shape {
            shape: ShapeEnum::Sphere(Sphere::new(id)),
            ..Default::default()
        }
    }
    pub fn plane() -> Shape {
        Shape {
            shape: ShapeEnum::Plane(Plane::new()),
            ..Default::default()
        }
    }

    pub fn set_transform(&mut self, m: &Matrix4) {
        self.transform = *m;
        self.inverse_transform = self.transform.inverse();
    }

    pub fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    pub fn inverse_transform(&self) -> &Matrix4 {
        &self.inverse_transform
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        // Why multiply by the inverse transpose?
        // https://stackoverflow.com/questions/13654401/why-transform-normals-with-the-transpose-of-the-inverse-of-the-modelview-matrix
        let inverse_transform = inverse(&self.transform);
        let local_point = inverse_transform * world_point;
        let local_normal = self.shape.local_normal_at(&local_point);
        let mut world_normal = transpose(&inverse_transform) * local_normal;
        world_normal.set_w(0.0);
        normalize(&world_normal)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
}

impl Default for ShapeEnum {
    fn default() -> Self {
        ShapeEnum::Sphere(Sphere::new(0))
    }
}

pub trait ShapeTrait {
    fn local_intersect(&self, local_ray: &Ray) -> Intersections;
    fn local_normal_at(&self, local_point: &Point) -> Vector;
}

impl ShapeTrait for ShapeEnum {
    fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        match self {
            ShapeEnum::Sphere(ref sphere) => sphere.local_intersect(local_ray),
            ShapeEnum::Plane(ref plane) => plane.local_intersect(local_ray),
        }
    }

    fn local_normal_at(&self, local_point: &Point) -> Vector {
        match self {
            ShapeEnum::Sphere(ref sphere) => sphere.local_normal_at(local_point),
            ShapeEnum::Plane(ref plane) => plane.local_normal_at(local_point),
        }
    }
}

pub fn normal_at(object: &Shape, world_point: &Point) -> Vector {
    object.normal_at(world_point)
}

pub fn sphere(id: i32) -> Shape {
    Shape::sphere(id)
}

pub fn plane() -> Shape {
    Shape::plane()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::intersections::intersect;
    use crate::materials::default_material;
    use crate::matrices::identity4;
    use crate::rays::ray;
    use crate::transformations::{rotation_z, scaling, translation};
    use crate::tuples::{point, vector};
    use approx::assert_relative_eq;
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    #[test]
    fn test_vec_of_shapes() {
        let v = vec![
            Shape {
                shape: ShapeEnum::Sphere(Sphere::new(1)),
                transform: identity4(),
                material: default_material(),
                ..Default::default()
            },
            Shape {
                shape: ShapeEnum::Plane(Plane::new()),
                transform: identity4(),
                material: default_material(),
                ..Default::default()
            },
        ];
        assert!(matches!(v[0].shape, ShapeEnum::Sphere { .. }));
        assert!(matches!(v[1].shape, ShapeEnum::Plane { .. }));
    }

    // Moved some tests from spheres.rs as they need to
    // work with transforms and materials.

    // A sphere's default transformation
    #[test]
    fn sphere_default_transformation() {
        let s = sphere(1);
        assert_eq!(s.transform, identity4());
    }

    // Changing a sphere's transformation
    #[test]
    fn changing_sphere_transformation() {
        let mut s = sphere(1);
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(&t);
        assert_eq!(s.transform, t);
    }

    // Intersecting a scaled sphere with a ray
    #[test]
    fn intersecting_a_scaled_sphere_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere(1);
        s.set_transform(&scaling(2.0, 2.0, 2.0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    // Intersecting a translated sphere with a ray
    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere(1);
        s.set_transform(&translation(5.0, 0.0, 0.0));
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    // Computing the normal on a translated sphere
    #[test]
    fn compute_normal_on_translated_sphere() {
        let mut s = sphere(1);
        s.set_transform(&translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.70711, -FRAC_1_SQRT_2));
        assert_relative_eq!(
            n,
            vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            epsilon = 1e-5
        );
    }

    // Computing the normal on a transformed sphere
    #[test]
    fn compute_normal_on_transformed_sphere() {
        let mut s = sphere(1);
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(&m);
        let k = f64::sqrt(2.0) / 2.0;
        //let n = s.shape.local_normal_at(&point(0.0, k, -k));
        let n = s.normal_at(&point(0.0, k, -k));
        assert_relative_eq!(n, vector(0.0, 0.97014, -0.24245), epsilon = 1e-4);
    }

    // A sphere has a default material
    #[test]
    fn sphere_has_default_material() {
        let s = sphere(1);
        let m = s.material;
        assert_eq!(m, default_material());
    }

    // A sphere may be assigned a material
    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = sphere(1);
        let mut m = default_material();
        m.ambient = 1.0;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }
}
