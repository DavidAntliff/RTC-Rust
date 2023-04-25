// Chapter 13 - Double-Napped Cone

use crate::intersections::{Intersection, Intersections};
use crate::math::EPSILON;
use crate::rays::Ray;
use crate::tuples::{vector, Point, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cone {
    pub minimum_y: f64,   // exclusive Y coordinates in object space
    pub maximum_y: f64,   // must be > minimum_y for anything to render
    pub closed_min: bool, // if true, cap is rendered
    pub closed_max: bool, // if true, cap is rendered
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            minimum_y: -f64::INFINITY,
            maximum_y: f64::INFINITY,
            closed_min: false,
            closed_max: false,
        }
    }
}

impl Cone {
    pub fn new() -> Self {
        Cone::default()
    }

    pub fn local_normal_at(&self, local_point: &Point) -> Vector {
        // Distances
        let x2 = local_point.x() * local_point.x();
        let z2 = local_point.z() * local_point.z();
        let y2 = local_point.y() * local_point.y();

        // End caps are the same as for a cylinder, except the radius depends on y:
        let dist = x2 + z2;
        if self.closed_max && dist < y2 && local_point.y() >= self.maximum_y - EPSILON {
            vector(0.0, 1.0, 0.0)
        } else if self.closed_min && dist < y2 && local_point.y() <= self.minimum_y + EPSILON {
            vector(0.0, -1.0, 0.0)
        } else {
            let y = if local_point.y() > 0.0 {
                -dist.sqrt()
            } else {
                dist.sqrt()
            };
            vector(local_point.x(), y, local_point.z())
        }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        let a = local_ray.direction.x() * local_ray.direction.x()
            - local_ray.direction.y() * local_ray.direction.y()
            + local_ray.direction.z() * local_ray.direction.z();
        let b = 2.0 * local_ray.origin.x() * local_ray.direction.x()
            - 2.0 * local_ray.origin.y() * local_ray.direction.y()
            + 2.0 * local_ray.origin.z() * local_ray.direction.z();

        let mut xs: Intersections = vec![];

        let a_is_zero = a.abs() < f64::EPSILON;

        if a_is_zero && b.abs() < f64::EPSILON {
            // miss, but might still hit cap...
        } else {
            let c = local_ray.origin.x() * local_ray.origin.x()
                - local_ray.origin.y() * local_ray.origin.y()
                + local_ray.origin.z() * local_ray.origin.z();

            if a_is_zero {
                // b isn't zero
                let t = -c / (2.0 * b);
                xs.push(Intersection::new(t, None));
            } else {
                let disc = b * b - 4.0 * a * c;

                // Ray intersects:
                if disc >= 0.0 {
                    let t0 = (-b - disc.sqrt()) / (2.0 * a);
                    let t1 = (-b + disc.sqrt()) / (2.0 * a);

                    // Check for truncation:
                    let y0 = local_ray.origin.y() + t0 * local_ray.direction.y();
                    if self.minimum_y < y0 && y0 < self.maximum_y {
                        xs.push(Intersection::new(t0, None));
                    }

                    let y1 = local_ray.origin.y() + t1 * local_ray.direction.y();
                    if self.minimum_y < y1 && y1 < self.maximum_y {
                        xs.push(Intersection::new(t1, None));
                    }
                }
            }
        }
        self.intersect_caps(local_ray, &mut xs);
        xs
    }

    fn intersect_caps(&self, ray: &Ray, xs: &mut Intersections) {
        // Caps only matter if the cylinder is closed and might be intersected
        if (!(self.closed_min || self.closed_max)) || (ray.direction.y().abs() < EPSILON) {
            return;
        }

        if self.closed_min {
            // Check for an intersection with the lower end cap by intersecting with
            // the plane at y = self.minimum_y
            let t = (self.minimum_y - ray.origin.y()) / ray.direction.y();
            if check_cap(ray, t, self.minimum_y.abs()) {
                xs.push(Intersection::new(t, None));
            }
        }

        if self.closed_max {
            // Check for an intersection with the upper end cap by intersecting with
            // the plane at y = self.maximum_y
            let t = (self.maximum_y - ray.origin.y()) / ray.direction.y();
            if check_cap(ray, t, self.maximum_y.abs()) {
                xs.push(Intersection::new(t, None));
            }
        }
    }
}

fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();
    x * x + z * z <= radius * radius
}

pub fn local_normal_at(c: &Cone, local_point: &Point) -> Vector {
    c.local_normal_at(local_point)
}

pub fn local_intersect<'a>(c: &'a Cone, local_ray: &Ray) -> Intersections<'a> {
    c.local_intersect(local_ray)
}

pub fn cone() -> Cone {
    Cone::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::ray;
    use crate::tuples::{point, vector, Point, Vector};
    use approx::assert_relative_eq;
    use rstest::rstest;

    struct TestItem {
        origin: Point,
        direction: Vector,
        t0: f64,
        t1: f64,
    }

    impl TestItem {
        fn new(origin: Point, direction: Vector, t0: f64, t1: f64) -> Self {
            TestItem {
                origin,
                direction,
                t0,
                t1,
            }
        }
    }

    // Intersecting a cone with a ray
    #[rstest]
    #[case(TestItem::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0))]
    #[case(TestItem::new(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0), 8.66025, 8.66025))]
    #[case(TestItem::new(point(1.0, 1.0, -5.0), vector(-0.5, -1.0, 1.0), 4.55006, 49.44994))]
    fn intersecting_cone_with_ray(#[case] item: TestItem) {
        let c = cone();
        let direction = item.direction.normalize();
        let r = ray(item.origin, direction);
        let xs = local_intersect(&c, &r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].t, item.t0, epsilon = 1e-5);
        assert_relative_eq!(xs[1].t, item.t1, epsilon = 1e-5);
    }

    // Intersecting a cone with a ray parallel to one of its halves
    #[test]
    fn intersecting_cone_with_ray_parallel_to_a_half() {
        let c = cone();
        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = ray(point(0.0, 0.0, -1.0), direction);
        let xs = local_intersect(&c, &r);
        assert_eq!(xs.len(), 1);
        assert_relative_eq!(xs[0].t, 0.35355, epsilon = 1e-5);
    }

    struct TestItem2 {
        origin: Point,
        direction: Vector,
        count: usize,
    }

    impl TestItem2 {
        fn new(origin: Point, direction: Vector, count: usize) -> Self {
            TestItem2 {
                origin,
                direction,
                count,
            }
        }
    }

    // Intersecting a cone's end caps
    #[rstest]
    #[case(TestItem2::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0), 0))]
    #[case(TestItem2::new(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2))]
    #[case(TestItem2::new(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0), 4))]
    fn intersecting_cone_end_caps(#[case] item: TestItem2) {
        let mut c = cone();
        c.minimum_y = -0.5;
        c.maximum_y = 0.5;
        c.closed_min = true;
        c.closed_max = true;
        let direction = item.direction.normalize();
        let r = ray(item.origin, direction);
        let xs = local_intersect(&c, &r);
        assert_eq!(xs.len(), item.count);
    }

    // Computing the normal vector on a cone
    #[rstest]
    #[case(TestItem2::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 0.0), 0))]
    #[case(TestItem2::new(point(1.0, 1.0, 1.0), vector(1.0, -f64::sqrt(2.0), 1.0), 0))]
    #[case(TestItem2::new(point(-1.0, -1.0, 0.0), vector(-1.0, 1.0, 0.0), 0))]
    fn compute_normal_vector_on_cone(#[case] item: TestItem2) {
        let c = cone();
        let n = local_normal_at(&c, &item.origin);
        assert_eq!(n, item.direction);
    }
}
