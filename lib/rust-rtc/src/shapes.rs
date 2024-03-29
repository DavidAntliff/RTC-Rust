// Chapter 9: Planes

use crate::cones::Cone;
use crate::cubes::Cube;
use crate::cylinders::Cylinder;
use crate::intersections::Intersections;
use crate::materials::{Material, RefractiveIndex};
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

    pub fn glass_sphere() -> Shape {
        let mut shape = Shape {
            shape: ShapeEnum::Sphere(Sphere::new(0)),
            ..Default::default()
        };
        shape.material.transparency = 1.0;
        shape.material.refractive_index = RefractiveIndex::GLASS;
        shape
    }

    pub fn plane() -> Shape {
        Shape {
            shape: ShapeEnum::Plane(Plane::new()),
            ..Default::default()
        }
    }

    pub fn cube() -> Shape {
        Shape {
            shape: ShapeEnum::Cube(Cube::new()),
            ..Default::default()
        }
    }

    pub fn cylinder(minimum_y: f64, maximum_y: f64, closed_min: bool, closed_max: bool) -> Shape {
        Shape {
            //shape: ShapeEnum::Cylinder(cyl),
            shape: ShapeEnum::Cylinder(Cylinder {
                minimum_y,
                maximum_y,
                closed_min,
                closed_max,
            }),
            ..Default::default()
        }
    }

    pub fn infinite_cylinder() -> Shape {
        Shape {
            shape: ShapeEnum::Cylinder(Cylinder::new()),
            ..Default::default()
        }
    }

    pub fn cone() -> Shape {
        Shape {
            shape: ShapeEnum::Cone(Cone {
                minimum_y: -1.0,
                maximum_y: 0.0,
                closed_min: true,
                closed_max: true,
            }),
            ..Default::default()
        }
    }

    // Functions to extract primitive type
    pub fn as_sphere_primitive(&mut self) -> Option<&mut Sphere> {
        match self.shape {
            ShapeEnum::Sphere(ref mut x) => Some(x),
            _ => None,
        }
    }

    pub fn as_cylinder_primitive(&mut self) -> Option<&mut Cylinder> {
        match self.shape {
            ShapeEnum::Cylinder(ref mut x) => Some(x),
            _ => None,
        }
    }

    pub fn as_cone_primitive(&mut self) -> Option<&mut Cone> {
        match self.shape {
            ShapeEnum::Cone(ref mut x) => Some(x),
            _ => None,
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
        let local_normal = self.local_normal_at(&local_point);
        let mut world_normal = transpose(&inverse_transform) * local_normal;
        world_normal.set_w(0.0);
        normalize(&world_normal)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
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

impl ShapeTrait for Shape {
    fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        self.shape.local_intersect(local_ray)
    }

    fn local_normal_at(&self, local_point: &Point) -> Vector {
        self.shape.local_normal_at(local_point)
    }
}

impl ShapeTrait for ShapeEnum {
    fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        match self {
            ShapeEnum::Sphere(ref sphere) => sphere.local_intersect(local_ray),
            ShapeEnum::Plane(ref plane) => plane.local_intersect(local_ray),
            ShapeEnum::Cube(ref cube) => cube.local_intersect(local_ray),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.local_intersect(local_ray),
            ShapeEnum::Cone(ref cone) => cone.local_intersect(local_ray),
        }
    }

    fn local_normal_at(&self, local_point: &Point) -> Vector {
        match self {
            ShapeEnum::Sphere(ref sphere) => sphere.local_normal_at(local_point),
            ShapeEnum::Plane(ref plane) => plane.local_normal_at(local_point),
            ShapeEnum::Cube(ref cube) => cube.local_normal_at(local_point),
            ShapeEnum::Cylinder(ref cylinder) => cylinder.local_normal_at(local_point),
            ShapeEnum::Cone(ref cone) => cone.local_normal_at(local_point),
        }
    }
}

pub fn normal_at(object: &Shape, world_point: &Point) -> Vector {
    object.normal_at(world_point)
}

pub fn sphere(id: i32) -> Shape {
    Shape::sphere(id)
}

pub fn glass_sphere() -> Shape {
    Shape::glass_sphere()
}

pub fn plane() -> Shape {
    Shape::plane()
}

pub fn cube() -> Shape {
    Shape::cube()
}

pub fn infinite_cylinder() -> Shape {
    Shape::infinite_cylinder()
}

pub fn cylinder(min_y: f64, max_y: f64, closed_min: bool, closed_max: bool) -> Shape {
    Shape::cylinder(min_y, max_y, closed_min, closed_max)
}

pub fn cone() -> Shape {
    Shape::cone()
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

    // Get access to internal primitive type
    #[test]
    fn get_primitive_shape() {
        let mut s = sphere(42);
        let primitive = match s.shape {
            ShapeEnum::Sphere(x) => x,
            _ => panic!("bad"),
        };
        assert_eq!(primitive.id, 42);

        let primitive2 = s.as_sphere_primitive();
        assert_eq!(primitive2.unwrap(), &primitive);
    }
}
