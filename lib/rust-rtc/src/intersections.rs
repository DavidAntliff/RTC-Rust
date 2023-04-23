// Chapter 5: Ray-Sphere Intersections

use crate::math::EPSILON;
use crate::rays::Ray;
use crate::shapes::{normal_at, Shape, ShapeTrait};
use crate::tuples::{dot, reflect, Point, Vector};

use crate::materials::RefractiveIndex;
pub use std::vec as intersections;

#[derive(Debug, PartialEq, Default, Copy, Clone)]
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
    let local_ray = ray.transform(object.inverse_transform());
    let mut intersections = object.shape.local_intersect(&local_ray);
    for mut intersection in &mut intersections {
        intersection.object = Some(object);
    }
    intersections
}

/// Given a vector of ray intersections, sort in ascending order by parameter t, and then
/// return the intersection with the lowest positive t.
pub fn hit<'a>(intersections: &'a mut Intersections<'a>) -> Option<&'a Intersection<'a>> {
    intersections.sort_by(|a, b| a.t.total_cmp(&b.t));

    let hit = intersections.iter().find(|&x| x.t > 0.0);
    hit
}

#[derive(Debug)]
pub struct IntersectionComputation<'a> {
    pub t: f64,
    pub object: &'a Shape,
    pub point: Point,
    pub under_point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub reflectv: Vector,
    pub n1: f64, // refractive index of material being exited
    pub n2: f64, // refractive index of material being entered
}

// Note to self: cannot implement Default for IntersectionComputation
// because it contains a reference field.

impl IntersectionComputation<'_> {
    pub fn new(object: &Shape) -> IntersectionComputation {
        IntersectionComputation {
            t: 0.0,
            object,
            point: Point::default(),
            under_point: Point::default(),
            over_point: Point::default(),
            eyev: Vector::default(),
            normalv: Vector::default(),
            inside: false,
            reflectv: Vector::default(),
            n1: RefractiveIndex::VACUUM,
            n2: RefractiveIndex::VACUUM,
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
    comps.normalv = normal_at(comps.object, &comps.point);

    if dot(&comps.normalv, &comps.eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = -comps.normalv;
    }

    comps.under_point = comps.point - comps.normalv * EPSILON;
    comps.over_point = comps.point + comps.normalv * EPSILON;

    comps.reflectv = reflect(&ray.direction, &comps.normalv);

    comps
}

pub fn prepare_computations_for_refraction<'a>(
    intersection: &'a Intersection,
    ray: &Ray,
    intersections: &[Intersection],
) -> IntersectionComputation<'a> {
    let mut comps = prepare_computations(intersection, ray);

    // Determine n1 (refractive index of material being exited),
    // and n2 (refractive index of material being entered):
    let mut containers: Vec<&Shape> = vec![];
    for i in intersections {
        let object = &i.object.expect("object should exist");

        if std::ptr::eq(i, intersection) {
            // If the intersection is the hit, set n1 to the refractive index of the last object
            // in the containers list. If the list is empty, set it to 1.0 (vacuum).
            comps.n1 = match containers.last() {
                Some(object) => object.material.refractive_index,
                None => RefractiveIndex::VACUUM,
            };
        }

        // If the intersection's object is already in the containers list, then the hit intersection
        // must be exiting the object. Remove it from the containers list.
        // Otherwise, the intersection is entering the object, so add to the containers list.
        let mut index = None;
        for (j, x) in containers.iter().enumerate() {
            if std::ptr::eq(*x, *object) {
                index = Some(j);
                break;
            }
        }
        match index {
            Some(n) => {
                containers.remove(n);
            }
            None => {
                containers.push(object);
            }
        }

        // If the intersection is the hit, set n2 to the refractive index of the last object
        // in the containers list. If the list is empty, then there is no containing object
        // and n2 should be set 1.0 (vacuum).
        if std::ptr::eq(i, intersection) {
            comps.n2 = match containers.last() {
                Some(object) => object.material.refractive_index,
                None => RefractiveIndex::VACUUM,
            };

            break;
        }
    }

    comps
}

// https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
pub fn schlick(comps: &IntersectionComputation) -> f64 {
    // Cosine of angle between eye and normal vector:
    let mut cos = dot(&comps.eyev, &comps.normalv);

    // Total internal reflection only possible if n1 > n2
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n * n * (1.0 - cos * cos);
        if sin2_t > 1.0 {
            return 1.0;
        }

        // Compute cosine of theta_t:
        let cos_t = f64::sqrt(1.0 - sin2_t);

        // When n1 > n2, use cos(theta_t) as 'cos'
        cos = cos_t;
    }

    let k = (comps.n1 - comps.n2) / (comps.n1 + comps.n2);
    let r0 = k * k;

    let w = 1.0 - cos;
    r0 + (1.0 - r0) * w * w * w * w * w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::ray;
    use crate::shapes::{glass_sphere, plane, sphere};
    use crate::transformations::{scaling, translation};
    use crate::tuples::{point, vector};
    use approx::assert_relative_eq;
    use rstest::rstest;

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

    // Chapter 11: Reflections

    // Precomputing the reflection vector
    #[test]
    fn precompute_reflection_vector() {
        let shape = plane();
        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 1.0, -1.0), vector(0.0, -k, k));
        let i = intersection(f64::sqrt(2.0), Some(&shape));
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.reflectv, vector(0.0, k, k));
    }

    // Finding n1 and n2 at various intersections
    #[rstest]
    #[case(0, 1.0, 1.5)]
    #[case(1, 1.5, 2.0)]
    #[case(2, 2.0, 2.5)]
    #[case(3, 2.5, 2.5)]
    #[case(4, 2.5, 1.5)]
    #[case(5, 1.5, 1.0)]
    fn finding_n1_and_n2_at_various_intersections(
        #[case] index: usize,
        #[case] n1: f64,
        #[case] n2: f64,
    ) {
        let mut a = glass_sphere();
        a.set_transform(&scaling(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let mut b = glass_sphere();
        b.set_transform(&translation(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = glass_sphere();
        c.set_transform(&translation(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;
        let r = ray(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs = intersections!(
            Intersection::new(2.0, Some(&a)),
            Intersection::new(2.75, Some(&b)),
            Intersection::new(3.25, Some(&c)),
            Intersection::new(4.75, Some(&b)),
            Intersection::new(5.25, Some(&c)),
            Intersection::new(6.0, Some(&a))
        );
        let comps = prepare_computations_for_refraction(&xs[index], &r, &xs);
        assert_eq!(comps.n1, n1);
        assert_eq!(comps.n2, n2);
    }

    // The under point is offset below the surface
    #[test]
    fn under_point_is_offset_below_surface() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = glass_sphere();
        shape.set_transform(&translation(0.0, 0.0, 1.0));
        let i = intersection(5.0, Some(&shape));
        let xs = intersections!(i);
        let comps = prepare_computations_for_refraction(&i, &r, &xs);
        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    // The Schlick approximation under total internal reflection
    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 0.0, k), vector(0.0, 1.0, 0.0));
        let xs = intersections!(
            Intersection::new(-k, Some(&shape)),
            Intersection::new(k, Some(&shape))
        );
        let comps = prepare_computations_for_refraction(&xs[1], &r, &xs);
        let reflectance = schlick(&comps);
        assert_eq!(reflectance, 1.0);
    }

    // The Schlick approximation with a perpendicular viewing angle
    #[test]
    fn schlick_approximation_with_perpendicular_view_angle() {
        let mut shape = glass_sphere();
        shape.material.refractive_index = 1.5; // tests assume glass_sphere() uses ri = 1.5
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = intersections!(
            Intersection::new(-1.0, Some(&shape)),
            Intersection::new(1.0, Some(&shape))
        );
        let comps = prepare_computations_for_refraction(&xs[1], &r, &xs);
        let reflectance = schlick(&comps);
        assert_relative_eq!(reflectance, 0.04, epsilon = 1e-5);
    }

    // The Schlick approximation with a small angle and n2 > n1
    #[test]
    fn schlick_approximation_with_small_angle_n2_gt_n1() {
        let mut shape = glass_sphere();
        shape.material.refractive_index = 1.5; // tests assume glass_sphere() uses ri = 1.5
        let r = ray(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0));
        let xs = intersections!(Intersection::new(1.8589, Some(&shape)));
        let comps = prepare_computations_for_refraction(&xs[0], &r, &xs);
        let reflectance = schlick(&comps);
        assert_relative_eq!(reflectance, 0.48873, epsilon = 1e-5);
    }
}
