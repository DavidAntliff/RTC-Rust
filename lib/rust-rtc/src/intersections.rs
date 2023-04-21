// Chapter 5: Ray-Sphere Intersections

use crate::math::EPSILON;
use crate::matrices::inverse;
use crate::rays::{Ray};
use crate::shapes::{normal_at, Shape, ShapeTrait};
use crate::tuples::{dot, Point, Vector};

pub use std::vec as intersections;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: Option<&'a Shape>,
}

impl Intersection<'_> {
    pub fn new(t: f64, object: Option<&Shape>) -> Intersection {
        Intersection { t, object }
    }
}

pub fn intersection(t: f64, object: Option<&Shape>) -> Intersection {
    Intersection::new(t, object)
}

pub type Intersections<'a> = Vec<Intersection<'a>>;

pub fn intersect<'a>(object: &'a Shape, ray: &Ray) -> Intersections<'a> {
    // Apply the inverse of the shape's transformation
    let local_ray = ray.transform(&inverse(object.transform()));
    let mut intersections = object.shape.local_intersect(&local_ray);
    for mut intersection in &mut intersections {
        intersection.object = Some(object);
    }
    intersections
}

pub fn hit<'a>(intersections: &'a mut Intersections<'a>) -> Option<&'a Intersection<'a>> {
    intersections.sort_by(|a, b| a.t.total_cmp(&b.t));

    let hit = intersections.iter().find(|&x| x.t > 0.0);
    hit
}

pub struct IntersectionComputation<'a> {
    pub t: f64,
    pub object: &'a Shape,
    pub point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

impl IntersectionComputation<'_> {
    pub fn new(object: &Shape) -> IntersectionComputation {
        IntersectionComputation {
            t: 0.0,
            object,
            point: Point::default(),
            over_point: Point::default(),
            eyev: Vector::default(),
            normalv: Vector::default(),
            inside: false,
        }
    }
}

pub fn prepare_computations<'a>(
    intersection: &'a Intersection,
    ray: &Ray,
) -> IntersectionComputation<'a> {
    let mut comps = IntersectionComputation::new(intersection.object.expect("no shape ref"));
    comps.t = intersection.t;

    comps.point = ray.position(comps.t);
    comps.eyev = -ray.direction;
    //comps.normalv = normal_at(comps.object.shape, &comps.point);
    comps.normalv = normal_at(comps.object, &comps.point);

    if dot(&comps.normalv, &comps.eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = -comps.normalv;
    }

    comps.over_point = comps.point + comps.normalv * EPSILON;

    comps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::ray;
    use crate::shapes::sphere;
    use crate::transformations::translation;
    use crate::tuples::{point, vector};

    // An intersection encapsulates t and object
    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = sphere(1);
        let i = intersection(3.5, Some(&s));
        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object.unwrap(), &s));
    }

    // Aggregating intersections
    #[test]
    fn aggregating_intersections() {
        let s = sphere(1);
        let i1 = intersection(1.0, Some(&s));
        let i2 = intersection(2.0, Some(&s));
        let xs = intersections!(i1, i2);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }
    // The hit, when all intersections have positive t
    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = sphere(1);
        let i1 = intersection(1.0, Some(&s));
        let i2 = intersection(2.0, Some(&s));
        let mut xs = intersections!(i2, i1);
        let i = hit(&mut xs);
        assert_eq!(i, Some(&i1));
    }

    // The hit, when some intersections have negative t
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere(1);
        let i1 = intersection(-1.0, Some(&s));
        let i2 = intersection(1.0, Some(&s));
        let mut xs = intersections!(i2, i1);
        let i = hit(&mut xs);
        assert_eq!(i, Some(&i2));
    }

    // The hit, when all intersections have negative t
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = sphere(1);
        let i1 = intersection(-2.0, Some(&s));
        let i2 = intersection(-1.0, Some(&s));
        let mut xs = intersections!(i2, i1);
        let i = hit(&mut xs);
        assert_eq!(i, None);
    }

    // The hit is always the lowest nonnegative intersection
    #[test]
    fn the_hit_is_always_the_lowest_non_negative() {
        let s = sphere(1);
        let i1 = intersection(5.0, Some(&s));
        let i2 = intersection(7.0, Some(&s));
        let i3 = intersection(-3.0, Some(&s));
        let i4 = intersection(2.0, Some(&s));
        let mut xs = intersections!(i1, i2, i3, i4);
        let i = hit(&mut xs);
        assert_eq!(i, Some(&i4));
    }

    // Precomputing the state of an intersection
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere(1);
        let i = intersection(4.0, Some(&shape));
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object.expect("no shape"));
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    // The hit, when an intersection occurs on the outside
    #[test]
    fn the_hit_when_intersection_occurs_on_outside() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = sphere(1);
        let i = intersection(4.0, Some(&shape));
        let comps = prepare_computations(&i, &r);
        assert!(!comps.inside);
    }

    // The hit, when an intersection occurs on the inside
    #[test]
    fn the_hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = sphere(1);
        let i = intersection(1.0, Some(&shape));
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        // normal would have been (0, 0, 1), but is inverted:
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    // The hit should offset the point
    #[test]
    fn hit_should_offset_point() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = sphere(1);
        shape.set_transform(&translation(0.0, 0.0, 1.0));
        let i = intersection(5.0, Some(&shape));
        let comps = prepare_computations(&i, &r);
        assert!(comps.over_point.z() < -EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }
}
