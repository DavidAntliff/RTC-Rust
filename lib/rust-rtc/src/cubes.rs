use crate::intersections::{intersections, Intersection, Intersections};
use crate::rays::Ray;
use crate::tuples::{vector, Point, Vector};

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Cube {}

impl Cube {
    pub fn new() -> Self {
        Cube {}
    }

    pub fn local_normal_at(&self, local_point: &Point) -> Vector {
        // Corners are treated as being on the +x or -x faces

        // Determine the face by the largest absolute value of the point component
        let maxc = f64::max(
            f64::max(local_point.x().abs(), local_point.y().abs()),
            local_point.z().abs(),
        );
        if maxc == local_point.x().abs() {
            vector(local_point.x(), 0.0, 0.0)
        } else if maxc == local_point.y().abs() {
            vector(0.0, local_point.y(), 0.0)
        } else {
            vector(0.0, 0.0, local_point.z())
        }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        let (xtmin, xtmax) = check_axis(local_ray.origin.x(), local_ray.direction.x());
        let (ytmin, ytmax) = check_axis(local_ray.origin.y(), local_ray.direction.y());

        // Optimisation: Early exit on miss.
        // If the maximum min value is already greater than the minimum max value, then
        // the contribution from the Z axis can only make this condition stronger.
        let xymin = f64::max(xtmin, ytmin);
        let xymax = f64::min(xtmax, ytmax);
        if xymin > xymax {
            return intersections![];
        }

        // However, the ray may still miss, so we must test Z as well:
        let (ztmin, ztmax) = check_axis(local_ray.origin.z(), local_ray.direction.z());

        let tmin = f64::max(xymin, ztmin);
        let tmax = f64::min(xymax, ztmax);

        if tmin > tmax {
            intersections![]
        } else {
            intersections!(
                Intersection {
                    t: tmin,
                    object: None
                },
                Intersection {
                    t: tmax,
                    object: None
                }
            )
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax) = if f64::abs(direction) >= f64::EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * f64::INFINITY,
            tmax_numerator * f64::INFINITY,
        )
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

pub fn cube() -> Cube {
    Cube::new()
}

pub fn local_intersect<'a>(c: &'a Cube, local_ray: &Ray) -> Intersections<'a> {
    c.local_intersect(local_ray)
}

pub fn local_normal_at(c: &Cube, local_point: &Point) -> Vector {
    c.local_normal_at(local_point)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rays::ray;
    use crate::tuples::{point, vector, Point, Vector};
    use rstest::rstest;

    struct TestItem {
        origin: Point,
        direction: Vector,
        t1: f64,
        t2: f64,
    }

    impl TestItem {
        fn new(origin: Point, direction: Vector, t1: f64, t2: f64) -> Self {
            TestItem {
                origin,
                direction,
                t1,
                t2,
            }
        }
    }

    // A ray intersects a cube
    #[rstest]
    #[case(TestItem::new(point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0))] // +x
    #[case(TestItem::new(point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0))] // -x
    #[case(TestItem::new(point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0))] // +y
    #[case(TestItem::new(point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0))] // -y
    #[case(TestItem::new(point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0))] // +z
    #[case(TestItem::new(point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0))] // -z
    #[case(TestItem::new(point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0))] // inside
    fn ray_intersects_cube(#[case] item: TestItem) {
        let c = cube();
        let r = ray(item.origin, item.direction);
        let xs = local_intersect(&c, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, item.t1);
        assert_eq!(xs[1].t, item.t2);
    }

    // A ray misses a cube
    #[rstest]
    #[case(TestItem::new(point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018), 0.0, 0.0))]
    #[case(TestItem::new(point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345), 0.0, 0.0))]
    #[case(TestItem::new(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673), 0.0, 0.0))]
    #[case(TestItem::new(point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0), 0.0, 0.0))]
    #[case(TestItem::new(point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0), 0.0, 0.0))]
    fn ray_misses_cube(#[case] item: TestItem) {
        let c = cube();
        let r = ray(item.origin, item.direction);
        let xs = local_intersect(&c, &r);
        assert!(xs.is_empty());
    }

    // The normal on the surface of a cube
    #[rstest]
    #[case(TestItem::new(point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0), 0.0, 0.0))]
    #[case(TestItem::new(point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0), 0.0, 0.0))]
    #[case(TestItem::new(point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0), 0.0, 0.0))]
    #[case(TestItem::new(point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0), 0.0, 0.0))]
    fn normal_on_surface_of_cube(#[case] item: TestItem) {
        let c = cube();
        let p = item.origin;
        let normal = local_normal_at(&c, &p);
        assert_eq!(normal, item.direction);
    }
}
