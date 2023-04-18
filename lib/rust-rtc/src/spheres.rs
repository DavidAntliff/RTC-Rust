// Chapter 5: Ray-Sphere Intersections

use crate::intersections::{Intersection, intersections, Intersections};
use crate::materials::{default_material, Material};
use crate::matrices::{identity4, Matrix4};
use crate::rays::Ray;
use crate::tuples::{dot, normalize, point, Point, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub id: i32,
    //pub transform: Matrix4,
    //pub material: Material, // copy, for now
}

impl Sphere {
    pub fn new(id: i32) -> Sphere {
        Sphere {
            id,
            //transform: identity4(),
            //material: default_material(),
        }
    }

    // pub fn set_transform(&mut self, m: &Matrix4) {
    //     self.transform = *m;
    // }

    pub fn local_normal_at(&self, local_point: &Point) -> Vector {
        // Assume the point is always on the surface of the sphere
        let mut object_normal = local_point - point(0.0, 0.0, 0.0);
        object_normal.set_w(0.0);
        normalize(&object_normal)
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        // TODO: A more stable algorithm at:
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html

        // The vector from the sphere's centre, to the ray origin
        // Remember, the sphere is centred at the world origin
        let sphere_to_ray = local_ray.origin - point(0.0, 0.0, 0.0);

        let a = dot(&local_ray.direction, &local_ray.direction);
        let b = 2.0 * dot(&local_ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // miss
            return intersections!();
        }

        let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

        //intersections!(intersection(t1, None), intersection(t2, None))
        intersections!(Intersection {t: t1, object: None},
                       Intersection {t: t2, object: None})
    }
}

pub fn sphere(id: i32) -> Sphere {
    Sphere::new(id)
}

pub fn local_normal_at(s: &Sphere, local_point: &Point) -> Vector {
    s.local_normal_at(local_point)
}

pub fn local_intersect<'a>(s: &'a Sphere, local_ray: &Ray) -> Intersections<'a> {
    s.local_intersect(local_ray)
}

// pub fn set_transform(s: &mut Sphere, m: &Matrix4) {
//     s.set_transform(m);
// }



#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::ray;
    use crate::transformations::{rotation_z, scaling, translation};
    use crate::tuples::{point, vector};
    use std::f64::consts::{FRAC_1_SQRT_2, PI};
    use approx::assert_relative_eq;

    // A ray intersects a sphere at two points
    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere(1);
        let xs = local_intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    // A ray intersects a sphere at a tangent
    #[test]
    fn ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere(1);
        let xs = local_intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    // A ray misses a sphere
    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere(1);
        let xs = local_intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    // A ray originates inside a sphere
    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere(1);
        let xs = local_intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    // A sphere is behind a ray
    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere(1);
        let xs = local_intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    // Intersect sets the object on the intersection
    // FIXME: this test is no longer relevant with Shape?
    #[test]
    fn intersect_sets_the_object() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere(1);
        let xs = local_intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        //assert!(std::ptr::eq(xs[0].object.expect("no shape"), &s));
        //assert!(std::ptr::eq(xs[1].object.expect("no shape"), &s));
        // Local functions return None in the Option
        assert!(xs[0].object.is_none());
        assert!(xs[1].object.is_none());
    }

    // Chapter 6 - Light and Shading

    // The normal on a sphere at a point on the x axis
    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = sphere(1);
        let n = local_normal_at(&s, &point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    // The normal on a sphere at a point on the y axis
    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = sphere(1);
        let n = local_normal_at(&s, &point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    // The normal on a sphere at a point on the z axis
    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = sphere(1);
        let n = local_normal_at(&s, &point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    // The normal on a sphere at a non-axial point
    #[test]
    fn normal_on_sphere_at_non_axial_point() {
        let s = sphere(1);
        let k = f64::sqrt(3.0) / 3.0;
        let n = local_normal_at(&s, &point(k, k, k));
        assert_eq!(n, vector(k, k, k));
    }

    // The normal is a normalized vector
    #[test]
    fn normal_is_normalized_vector() {
        let s = sphere(1);
        let k = f64::sqrt(3.0) / 3.0;
        let n = local_normal_at(&s, &point(k, k, k));
        assert_eq!(n, normalize(&n));
    }


    /*
       TODO

       // A sphere has a default material
       #[test]
       fn sphere_has_default_material() {
           let s = sphere(1);
           let m = s.material();
           assert_eq!(m, material());
       }

       // A sphere may be assigned a material
       #[test]
       fn sphere_may_be_assigned_material() {
           let s = sphere(1);
           let m = material();
           m.set_ambient(1.0);
           s.set_material(m);
           assert_eq!(s.material(), m);
       }

    */
}
