use crate::intersections::{intersections, Intersection, Intersections};
use crate::rays::Ray;
use crate::tuples::{vector, Point, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cylinder {
    pub minimum_y: f64, // exclusive Y coordinates in object space
    pub maximum_y: f64,
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            minimum_y: -f64::INFINITY,
            maximum_y: f64::INFINITY,
        }
    }
}

impl Cylinder {
    pub fn new() -> Self {
        Cylinder::default()
    }

    pub fn local_normal_at(&self, local_point: &Point) -> Vector {
        // Normal on a cylinder surface point is equal to the vector to the point
        // projected onto the XZ plane:
        vector(local_point.x(), 0.0, local_point.z())
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        let a = local_ray.direction.x() * local_ray.direction.x()
            + local_ray.direction.z() * local_ray.direction.z();

        // Ray is parallel to the Y axis:
        if a < f64::EPSILON {
            return intersections![];
        }

        let b = 2.0 * local_ray.origin.x() * local_ray.direction.x()
            + 2.0 * local_ray.origin.z() * local_ray.direction.z();
        let c = local_ray.origin.x() * local_ray.origin.x()
            + local_ray.origin.z() * local_ray.origin.z()
            - 1.0;
        let disc = b * b - 4.0 * a * c;

        // Ray does not intersect:
        if disc < 0.0 {
            intersections![]
        } else {
            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);

            // Book (page 184) shows swap(t0, t1) but it breaks tests:
            // let (t0, t1) = if t0 > 1.0 {
            //     (t1, t0)
            // } else {
            //     (t0, t1)
            // };

            // Check for truncation:
            let mut xs: Intersections = vec![];

            let y0 = local_ray.origin.y() + t0 * local_ray.direction.y();
            if self.minimum_y < y0 && y0 < self.maximum_y {
                xs.push(Intersection::new(t0, None));
            }

            let y1 = local_ray.origin.y() + t1 * local_ray.direction.y();
            if self.minimum_y < y1 && y1 < self.maximum_y {
                xs.push(Intersection::new(t1, None));
            }

            xs
        }
    }
}

pub fn local_normal_at(c: &Cylinder, local_point: &Point) -> Vector {
    c.local_normal_at(local_point)
}

pub fn local_intersect<'a>(c: &'a Cylinder, local_ray: &Ray) -> Intersections<'a> {
    c.local_intersect(local_ray)
}

pub fn cylinder() -> Cylinder {
    Cylinder::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::rays::ray;
    use crate::tuples::{normalize, point, vector, Point, Vector};
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

    // A ray misses a cylinder
    #[rstest]
    #[case(TestItem::new(point(1.0, 0.0, 0.0), vector(0.0, 1.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0), 0.0, 0.0))]
    fn ray_misses_cylinder(#[case] item: TestItem) {
        let cyl = cylinder();
        let direction = normalize(&item.direction);
        let r = ray(item.origin, direction);
        let xs = local_intersect(&cyl, &r);
        assert!(xs.is_empty());
    }

    // A ray strikes a cylinder
    #[rstest]
    #[case(TestItem::new(point(1.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0))]
    #[case(TestItem::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0))]
    #[case(TestItem::new(point(0.5, 0.0, -5.0), vector(0.1, 1.0, 1.0), 6.80798, 7.08872))]
    fn ray_strikes_cylinder(#[case] item: TestItem) {
        let cyl = cylinder();
        let direction = normalize(&item.direction);
        let r = ray(item.origin, direction);
        let xs = local_intersect(&cyl, &r);
        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].t, item.t0, epsilon = 1e-5);
        assert_relative_eq!(xs[1].t, item.t1, epsilon = 1e-5);
    }

    // Normal vector on a cylinder
    #[rstest]
    #[case(TestItem::new(point(1.0, 0.0, 0.0), vector(1.0, 0.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0), 0.0, 0.0))]
    #[case(TestItem::new(point(0.0, -2.0, 1.0), vector(0.0, 0.0, 1.0), 0.0, 0.0))]
    #[case(TestItem::new(point(-1.0, 1.0, 0.0), vector(-1.0, 0.0, 0.0), 0.0, 0.0))]
    fn normal_vector_on_cylinder(#[case] item: TestItem) {
        let cyl = cylinder();
        let n = local_normal_at(&cyl, &item.origin);
        assert_eq!(n, item.direction);
    }

    // The default minimum and maximum for a cylinder
    #[test]
    fn default_minimum_and_maximum() {
        let cyl = cylinder();
        assert_eq!(cyl.minimum_y, -f64::INFINITY);
        assert_eq!(cyl.maximum_y, f64::INFINITY);
    }

    struct TestItem2 {
        point: Point,
        direction: Vector,
        count: usize,
    }

    impl TestItem2 {
        fn new(point: Point, direction: Vector, count: usize) -> Self {
            TestItem2 {
                point,
                direction,
                count,
            }
        }
    }

    #[rstest]
    #[case(TestItem2::new(point(0.0, 1.5, 0.0), vector(0.1, 1.0, 0.0), 0))]
    #[case(TestItem2::new(point(0.0, 3.0, -5.0), vector(0.0, 0.0, 1.0), 0))]
    #[case(TestItem2::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0))]
    #[case(TestItem2::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0), 0))]
    #[case(TestItem2::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0), 0))]
    #[case(TestItem2::new(point(0.0, 1.5, -2.0), vector(0.0, 0.0, 1.0), 2))]
    fn intersecting_constrained_cylinder(#[case] item: TestItem2) {
        let mut cyl = cylinder();
        cyl.minimum_y = 1.0;
        cyl.maximum_y = 2.0;
        let direction = item.direction.normalize();
        let r = ray(item.point, direction);
        let xs = local_intersect(&cyl, &r);
        assert_eq!(xs.len(), item.count);
    }
}
