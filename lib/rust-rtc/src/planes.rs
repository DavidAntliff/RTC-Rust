// Chapter 9 - Planes

use crate::intersections::{intersection, intersections, Intersections};
use crate::math::EPSILON;
use crate::rays::Ray;
use crate::tuples::{vector, Point, Vector};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }

    pub fn local_normal_at(&self, _local_point: &Point) -> Vector {
        // The normal always points in the positive Y direction
        vector(0.0, 1.0, 0.0)
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        // The plane is at the origin, extending infinitely in both X and Z directions.
        //
        // 4 cases:
        //   1. ray parallel to plane, never intersects
        //   2. ray coplanar with plane, treat as a miss
        //   3. ray origin is above the plane
        //   4. ray origin is below the plane

        if f64::abs(local_ray.direction.y()) < EPSILON {
            return intersections!();
        }

        let t = -local_ray.origin.y() / local_ray.direction.y();
        intersections!(intersection(t, None))
    }
}

pub fn plane() -> Plane {
    Plane::new()
}

pub fn local_normal_at(p: &Plane, local_point: &Point) -> Vector {
    p.local_normal_at(local_point)
}

pub fn local_intersect<'a>(p: &'a Plane, local_ray: &Ray) -> Intersections<'a> {
    p.local_intersect(local_ray)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::ray;
    use crate::tuples::point;

    // The normal of a plane is constant everywhere
    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = plane();
        let n1 = local_normal_at(&p, &point(0.0, 0.0, 0.0));
        let n2 = local_normal_at(&p, &point(10.0, 0.0, -10.0));
        let n3 = local_normal_at(&p, &point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    // Intersect with a ray parallel to the plane
    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = plane();
        let r = ray(point(0.0, 1.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = local_intersect(&p, &r);
        assert!(xs.is_empty());
    }

    // Intersect with a coplanar ray
    #[test]
    fn intersect_with_coplanar_ray() {
        let p = plane();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = local_intersect(&p, &r);
        assert!(xs.is_empty());
    }

    // A ray intersecting a plane from above
    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = plane();
        let r = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = local_intersect(&p, &r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        // Local functions return None in the Option
        assert!(xs[0].object.is_none());
    }

    // A ray intersecting a plane from below
    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = plane();
        let r = ray(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = local_intersect(&p, &r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        // Local functions return None in the Option
        assert!(xs[0].object.is_none());
    }
}
