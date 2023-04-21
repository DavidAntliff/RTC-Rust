// Chapter 7: Making a Scene

use crate::colors::{color, Color};
use crate::intersections::{
    hit, intersect, prepare_computations, IntersectionComputation, Intersections,
};
use crate::lights::{point_light, PointLight};
use crate::materials::material;
use crate::rays::{ray, Ray};
use crate::shapes::{sphere, Shape};
use crate::transformations::scaling;
use crate::tuples::{magnitude, normalize, point, Point};

#[derive(Default)]
pub struct World {
    light: Option<PointLight>,
    objects: Vec<Shape>,
}

impl World {
    fn new(light: PointLight, objects: Vec<Shape>) -> World {
        World {
            light: Some(light),
            objects,
        }
    }

    // TODO: support multiple lights
    pub fn add_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }

    pub fn add_object(&mut self, object: Shape) {
        self.objects.push(object);
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = Vec::with_capacity(2);

        // Intersections must be in sorted order
        for object in &self.objects {
            let xs = intersect(object, ray);
            // TODO: insert in sorted order?
            for i in xs {
                intersections.push(i);
            }
        }

        intersections.sort_by(|a, b| a.t.total_cmp(&b.t));
        intersections
    }

    fn is_shadowed(&self, point: &Point) -> bool {
        if let Some(light) = &self.light {
            let v = light.position - point;
            let distance = magnitude(&v);
            let direction = normalize(&v);

            let ray = ray(*point, direction);
            let mut intersections = intersect_world(self, &ray);

            if let Some(h) = hit(&mut intersections) {
                h.t < distance
            } else {
                false
            }
        } else {
            false // everything is in shadow
        }
    }

    // Returns the color at the intersection encapsulated by comps, in the given world.
    fn shade_hit(&self, comps: &IntersectionComputation, depth: i32) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);
        let surface = comps.object.material.lighting(
            comps.object,
            &self.light,
            &comps.over_point, // avoid boundary issues
            &comps.eyev,
            &comps.normalv,
            shadowed);
        let reflected = self.reflected_color(comps, depth);
        surface + reflected
    }

    fn color_at(&self, ray: &Ray, depth: i32) -> Color {
        let mut xs = intersect_world(self, ray);
        if let Some(i) = hit(&mut xs) {
            let comps = prepare_computations(i, ray);
            self.shade_hit(&comps, depth)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    fn reflected_color(&self, comps: &IntersectionComputation, depth: i32) -> Color {
        if comps.object.material.reflective == 0.0 || depth < 1 {
            color(0.0, 0.0, 0.0)
        } else {
            let reflected_ray = ray(comps.over_point, comps.reflectv);
            let reflected_color = self.color_at(&reflected_ray, depth - 1);
            reflected_color * comps.object.material.reflective
        }
    }
}

pub fn world() -> World {
    World::default()
}

pub fn default_world() -> World {
    let light = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

    let mut objects = vec![];

    let m = material(color(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0);
    let mut s1 = sphere(1);
    s1.material = m;
    objects.push(s1);

    let mut s2 = sphere(2);
    s2.set_transform(&scaling(0.5, 0.5, 0.5));
    objects.push(s2);

    World::new(light, objects)
}

pub fn intersect_world<'a>(world: &'a World, ray: &Ray) -> Intersections<'a> {
    world.intersect(ray)
}

pub fn is_shadowed(world: &World, point: &Point) -> bool {
    world.is_shadowed(point)
}

pub fn shade_hit(world: &World, comps: &IntersectionComputation, depth: i32) -> Color {
    world.shade_hit(comps, depth)
}

pub fn color_at(world: &World, ray: &Ray, depth: i32) -> Color {
    world.color_at(ray, depth)
}

pub fn reflected_color(world: &World, comps: &IntersectionComputation, depth: i32) -> Color {
    world.reflected_color(comps, depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersections::{intersection, prepare_computations};
    use crate::rays::ray;
    use crate::transformations::translation;
    use crate::tuples::vector;
    use approx::assert_relative_eq;
    use crate::shapes::plane;

    // Creating an empty world
    #[test]
    fn creating_an_empty_world() {
        let w = world();
        assert!(w.light.is_none());
        assert!(w.objects.is_empty());
    }

    // Creating the default world
    #[test]
    fn creating_the_default_world() {
        let light = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        let m = material(color(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0);
        let mut s1 = sphere(1);
        s1.material = m;
        let mut s2 = sphere(2);
        s2.set_transform(&scaling(0.5, 0.5, 0.5));

        let w = default_world();
        assert_eq!(w.light, Some(light));
        assert_eq!(w.objects[0], s1);
        assert_eq!(w.objects[1], s2);
    }

    // Intersect a world with a ray
    #[test]
    fn intersect_world_with_ray() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = intersect_world(&w, &r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    // Shading an intersection
    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = intersection(4.0, Some(shape));
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps, 1);
        assert_relative_eq!(c, color(0.38066, 0.47583, 0.2855), epsilon = 1e-5);
    }

    // Shading an intersection from the inside
    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = default_world();
        w.light = Some(point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0)));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = intersection(0.5, Some(shape));
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps, 1);
        assert_relative_eq!(c, color(0.90498, 0.90498, 0.90498), epsilon = 1e-5);
    }

    // The color when a ray misses
    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = color_at(&w, &r, 1);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    // The color when a ray hits
    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = color_at(&w, &r, 1);
        assert_relative_eq!(c, color(0.38066, 0.47583, 0.2855), epsilon = 1e-5);
    }

    // The color with an intersection behind the ray
    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = default_world();
        {
            let mut outer = &mut w.objects[0];
            outer.material.ambient = 1.0;
            let mut inner = &mut w.objects[1];
            inner.material.ambient = 1.0;
        }
        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = color_at(&w, &r, 1);
        assert_eq!(c, w.objects[1].material.color);
    }

    // There is no shadow when nothing is collinear with point and light
    #[test]
    fn no_shadow_when_nothing_between_point_and_light() {
        let w = default_world();
        let p = point(0.0, 10.0, 0.0);
        assert!(!is_shadowed(&w, &p));
    }

    // The shadow when an object is between the point and the light
    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);
        assert!(is_shadowed(&w, &p));
    }

    // There is no shadow when an object is behind the light
    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);
        assert!(!is_shadowed(&w, &p));
    }

    // There is no shadow when an object is behind the point
    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);
        assert!(!is_shadowed(&w, &p));
    }

    // shade_hit() is given an intersection in shadow
    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let mut w = world();
        w.add_light(point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0)));
        let s1 = sphere(1);
        w.add_object(s1);
        let mut s2 = sphere(2);
        s2.set_transform(&translation(0.0, 0.0, 10.0));
        w.add_object(s2.clone());
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(4.0, Some(&s2));
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps, 1);
        assert_eq!(c, color(0.1, 0.1, 0.1));
    }

    // Chapter 11: Reflections

    // The reflected color for a non-reflective material
    #[test]
    fn reflected_color_for_non_reflective_material() {
        let mut w = default_world();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        {
            let mut shape = &mut w.objects[1];
            shape.material.ambient = 1.0;
        }
        let shape = &w.objects[1];
        let i = intersection(1.0, Some(shape));
        let comps = prepare_computations(&i, &r);
        let color_ = reflected_color(&w, &comps, 1);
        assert_eq!(color_, color(0.0, 0.0, 0.0));
    }

    // The reflected color for a reflective material
    #[test]
    fn reflected_color_for_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&translation(0.0, -1.0, 0.0));
        w.add_object(shape);
        let shape = w.objects.last().expect("vec should not be empty");
        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -k, k));
        let i = intersection(f64::sqrt(2.0), Some(shape));
        let comps = prepare_computations(&i, &r);
        let color_ = reflected_color(&w, &comps, 1);
        assert_relative_eq!(color_, color(0.19032, 0.2379, 0.14274), epsilon=1e-4);
    }

    // shade_hit() with a reflective material
    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&translation(0.0, -1.0, 0.0));
        w.add_object(shape);
        let shape = w.objects.last().expect("vec should not be empty");
        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -k, k));
        let i = intersection(f64::sqrt(2.0), Some(shape));
        let comps = prepare_computations(&i, &r);
        let color_ = shade_hit(&w, &comps, 1);
        assert_relative_eq!(color_, color(0.87677, 0.92436, 0.82918), epsilon=1e-4);
    }

    // color_at() with mutually reflective surfaces
    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = world();
        w.add_light(point_light(point(0.0, 0.0, 0.0), color(1.0, 1.0, 1.0)));
        let mut lower = plane();
        lower.material.reflective = 1.0;
        lower.set_transform(&translation(0.0, -1.0, 0.0));
        w.add_object(lower);
        let mut upper = plane();
        upper.material.reflective = 1.0;
        upper.set_transform(&translation(0.0, 1.0, 0.0));
        w.add_object(upper);
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        println!("{:?}", color_at(&w, &r, 1));
    }

    // The reflected color at the maximum recursive depth
    #[test]
    fn reflected_color_at_maximum_recursive_depth() {
        let mut w = default_world();
        let mut shape = plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&translation(0.0, -1.0, 0.0));
        w.add_object(shape);
        let shape = w.objects.last().expect("vec should not be empty");
        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -k, k));
        let i = intersection(f64::sqrt(2.0), Some(shape));
        let comps = prepare_computations(&i, &r);
        let color_ = reflected_color(&w, &comps, 0);
        assert_eq!(color_, color(0.0, 0.0, 0.0));
    }
}
