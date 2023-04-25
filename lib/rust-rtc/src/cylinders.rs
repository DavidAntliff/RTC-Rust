use crate::intersections::{intersections, Intersection, Intersections};
use crate::rays::Ray;
use crate::tuples::{vector, Point, Vector};

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Cylinder {}

impl Cylinder {
    pub fn new() -> Self {
        Cylinder {}
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
            intersections!(Intersection::new(t0, None), Intersection::new(t1, None))
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
}
