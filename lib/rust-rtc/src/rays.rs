// Chapter 5: Ray-Sphere Intersections

use crate::matrices::Matrix4;
use crate::tuples::{Point, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: &Matrix4) -> Ray {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

pub fn ray(origin: Point, direction: Vector) -> Ray {
    Ray::new(origin, direction)
}

pub fn position(ray: &Ray, t: f64) -> Point {
    ray.position(t)
}

pub fn transform(ray: &Ray, m: &Matrix4) -> Ray {
    ray.transform(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{scaling, translation};
    use crate::tuples::{point, vector};

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = ray(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    // Computing a point from a distance
    #[test]
    fn compute_a_point_from_distance() {
        let r = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(position(&r, 0.0), point(2.0, 3.0, 4.0));
        assert_eq!(position(&r, 1.0), point(3.0, 3.0, 4.0));
        assert_eq!(position(&r, -1.0), point(1.0, 3.0, 4.0));
        assert_eq!(position(&r, 2.5), point(4.5, 3.0, 4.0));
    }

    // Translating a ray
    #[test]
    fn translating_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = transform(&r, &m);
        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    // Scaling a ray
    #[test]
    fn scaling_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = transform(&r, &m);
        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
}
