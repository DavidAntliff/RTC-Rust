// Chapter 7: Making a Scene

use anyhow::{anyhow, Result};

use crate::colors::{color, Color};
use crate::intersections::{
    intersect, prepare_computations_for_refraction, schlick, Intersection, IntersectionComputation,
    Intersections,
};
use crate::lights::{point_light, PointLight};
use crate::materials::material;
use crate::rays::{ray, Ray};
use crate::shapes::{sphere, Shape};
use crate::transformations::scaling;
use crate::tuples::{dot, magnitude, normalize, point, Point};

#[derive(Default, Debug, PartialEq)]
pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<Shape>,
}

#[derive(Debug)]
pub struct LightIndex(usize);

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectIndex(usize);

impl World {
    fn new(lights: Vec<PointLight>, objects: Vec<Shape>) -> World {
        World { lights, objects }
    }

    pub fn add_light(&mut self, light: PointLight) -> LightIndex {
        self.lights.push(light);
        LightIndex(self.lights.len() - 1)
    }

    pub fn add_object(&mut self, object: Shape) -> ObjectIndex {
        self.objects.push(object);
        ObjectIndex(self.objects.len() - 1)
    }

    fn validate_object_index(&self, idx: &ObjectIndex) -> Result<()> {
        if idx.0 >= self.objects.len() {
            Err(anyhow!("Index {} out of bounds", idx.0).into())
        } else {
            Ok(())
        }
    }

    pub fn get_object_ref(&self, idx: &ObjectIndex) -> &Shape {
        self.validate_object_index(idx).unwrap();
        &self.objects[idx.0]
    }

    pub fn add_child(
        &mut self,
        group_index: &ObjectIndex,
        object_index: &ObjectIndex,
    ) -> Result<()> {
        self.validate_object_index(group_index)?;
        self.validate_object_index(object_index)?;

        let group = &mut self.objects[group_index.0];
        group
            .as_group_primitive_mut()
            .ok_or(anyhow!("Not a group"))?
            .members
            .push(object_index.clone());

        let object = &mut self.objects[object_index.0];
        object.parent = Some(group_index.clone());
        Ok(())
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = Vec::with_capacity(2);

        // Intersections must be in sorted order
        for object in &self.objects {
            let xs = intersect(object, ray, Some(self));
            // TODO: insert in sorted order?
            for i in xs {
                intersections.push(i);
            }
        }

        intersections.sort_by(|a, b| a.t.total_cmp(&b.t));
        intersections
    }

    fn is_shadowed(&self, point: &Point, light: &PointLight) -> bool {
        // Cast a ray from this point to the light source
        let v = light.position - point;
        let distance = magnitude(&v);
        let direction = normalize(&v);

        let ray = ray(*point, direction);
        let intersections = intersect_world(self, &ray);

        // Filter out any objects that don't cast shadows
        let xs: Vec<Intersection> = intersections
            .into_iter()
            .filter(|x| x.object.expect("should be object").material.casts_shadow)
            .collect();

        // No need to call hit() as already sorted
        //if let Some(h) = hit(&mut xs) {
        let hit = xs.iter().find(|&x| x.t > 0.0);
        if let Some(h) = hit {
            h.t < distance
        } else {
            false
        }
    }

    // Returns the color at the intersection encapsulated by comps, in the given world.
    fn shade_hit(&self, comps: &IntersectionComputation, depth: i32) -> Color {
        let mut surface = color(0.0, 0.0, 0.0);

        for light in &self.lights {
            let shadowed =
                comps.object.material.receives_shadow && self.is_shadowed(&comps.over_point, light);
            let surface_from_light = comps.object.material.lighting(
                comps.object,
                &Some(*light),
                &comps.over_point, // avoid boundary issues
                &comps.eyev,
                &comps.normalv,
                shadowed,
            );
            surface += surface_from_light;
        }

        let reflected = self.reflected_color(comps, depth);
        let refracted = self.refracted_color(comps, depth);

        // Experimental: reduce surface color for reflective materials
        // (Makes reflective objects very dark)
        //let surface = surface * (1.0 - comps.object.material.reflective);

        if comps.object.material.reflective > 0.0 && comps.object.material.transparency > 0.0 {
            let reflectance = schlick(comps);
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    fn color_at(&self, ray: &Ray, depth: i32) -> Color {
        let xs = self.intersect(ray);

        // Sort & Find copied from intersections.hit(), due to borrowing issue
        // No need to sort as self.intersect() already does this.
        //xs.sort_by(|a, b| a.t.total_cmp(&b.t));
        let hit = xs.iter().find(|&x| x.t > 0.0);

        if let Some(i) = hit {
            let comps = prepare_computations_for_refraction(i, ray, &xs);
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

    fn refracted_color(&self, comps: &IntersectionComputation, depth: i32) -> Color {
        if comps.object.material.transparency == 0.0 || depth < 1 {
            color(0.0, 0.0, 0.0)
        } else {
            // Snell's law:  sin(theta_i) / sin(theta_t) = n2 / n1,
            // where theta_i is angle of the incoming ray, and theta_t is the angle of the refracted ray
            // Find theta_i, given theta_t, n1, n2:
            let n_ratio = comps.n1 / comps.n2;

            // Use fact that cos(theta_i) == dot(eye_vector, normal_vector)
            let cos_i = dot(&comps.eyev, &comps.normalv);

            // Find sin(theta_2)^2 via trig identity:
            let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

            // If sin2_t > 1.0, there is no transmission - Total Internal Reflection
            if sin2_t > 1.0 {
                return color(0.0, 0.0, 0.0);
            }

            // Find cos(theta_t) via trig identity:
            let cos_t = f64::sqrt(1.0 - sin2_t);

            // Compute direction of refracted ray
            let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

            let refracted_ray = ray(comps.under_point, direction);

            self.color_at(&refracted_ray, depth - 1) * comps.object.material.transparency
        }
    }
}

pub fn world() -> World {
    World::default()
}

pub fn default_world() -> World {
    let mut lights = vec![];
    let light = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
    lights.push(light);

    let mut objects = vec![];

    let m = material(color(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
    let mut s1 = sphere(1);
    s1.material = m;
    objects.push(s1);

    let mut s2 = sphere(2);
    s2.set_transform(&scaling(0.5, 0.5, 0.5));
    objects.push(s2);

    World::new(lights, objects)
}

pub fn intersect_world<'a>(world: &'a World, ray: &Ray) -> Intersections<'a> {
    world.intersect(ray)
}

pub fn is_shadowed(world: &World, point: &Point, light: &PointLight) -> bool {
    world.is_shadowed(point, light)
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

pub fn refracted_color(world: &World, comps: &IntersectionComputation, depth: i32) -> Color {
    world.refracted_color(comps, depth)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::intersections::{
        intersection, intersections, prepare_computations, prepare_computations_for_refraction,
        Intersection,
    };
    use crate::patterns::test_pattern;
    use crate::rays::ray;
    use crate::shapes::{group, plane, ShapeTrait};
    use crate::transformations::translation;
    use crate::tuples::vector;

    use super::*;

    // Creating an empty world
    #[test]
    fn creating_an_empty_world() {
        let w = world();
        assert!(w.lights.is_empty());
        assert!(w.objects.is_empty());
    }

    // Creating the default world
    #[test]
    fn creating_the_default_world() {
        let light = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        let m = material(color(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0);
        let mut s1 = sphere(1);
        s1.material = m;
        let mut s2 = sphere(2);
        s2.set_transform(&scaling(0.5, 0.5, 0.5));

        let w = default_world();
        assert_eq!(w.lights[0], light);
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
        w.lights = vec![];
        w.add_light(point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0)));
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
            let outer = &mut w.objects[0];
            outer.material.ambient = 1.0;
            let inner = &mut w.objects[1];
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
        assert!(!is_shadowed(&w, &p, &w.lights[0]));
    }

    // The shadow when an object is between the point and the light
    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);
        assert!(is_shadowed(&w, &p, &w.lights[0]));
    }

    // There is no shadow when an object is behind the light
    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);
        assert!(!is_shadowed(&w, &p, &w.lights[0]));
    }

    // There is no shadow when an object is behind the point
    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);
        assert!(!is_shadowed(&w, &p, &w.lights[0]));
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

    // shade_hit() is given an intersection in shadow, but material does not cast shadows
    #[test]
    fn shade_hit_given_intersection_in_shadow_but_material_does_not_cast_shadows() {
        let mut w = world();
        w.add_light(point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0)));
        let mut s1 = sphere(1);
        s1.material.casts_shadow = false;
        w.add_object(s1);
        let mut s2 = sphere(2);
        s2.set_transform(&translation(0.0, 0.0, 10.0));
        s2.material.casts_shadow = false;
        w.add_object(s2.clone());
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(4.0, Some(&s2));
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps, 1);
        assert_eq!(c, color(1.9, 1.9, 1.9));
    }

    // Chapter 11: Reflections

    // The reflected color for a non-reflective material
    #[test]
    fn reflected_color_for_non_reflective_material() {
        let mut w = default_world();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        {
            let shape = &mut w.objects[1];
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
        assert_relative_eq!(color_, color(0.19032, 0.2379, 0.14274), epsilon = 1e-4);
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
        assert_relative_eq!(color_, color(0.87677, 0.92436, 0.82918), epsilon = 1e-4);
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

    // The refracted color with an opaque surface
    #[test]
    fn refracted_color_with_opaque_surface() {
        let w = default_world();
        let shape = &w.objects[0];
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = intersections!(
            Intersection::new(4.0, Some(shape)),
            Intersection::new(6.0, Some(shape))
        );
        let comps = prepare_computations_for_refraction(&xs[0], &r, &xs);
        let c = refracted_color(&w, &comps, 5);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    // The refracted color at the maximum recursive depth
    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let mut w = default_world();
        {
            let shape = &mut w.objects[0];
            shape.material.transparency = 1.0;
            shape.material.refractive_index = 1.5;
        }
        let shape = &w.objects[0];
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = intersections!(
            Intersection::new(4.0, Some(shape)),
            Intersection::new(6.0, Some(shape))
        );
        let comps = prepare_computations_for_refraction(&xs[0], &r, &xs);
        let c = refracted_color(&w, &comps, 0);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    // The refracted color under total internal reflection
    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = default_world();
        {
            let shape = &mut w.objects[0];
            shape.material.transparency = 1.0;
            shape.material.refractive_index = 1.5;
        }
        let shape = &w.objects[0];
        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 0.0, k), vector(0.0, 1.0, 0.0));
        let xs = intersections!(
            Intersection::new(-k, Some(shape)),
            Intersection::new(k, Some(shape))
        );
        // Since we're inside the sphere, need to look at the *second* intersection: xs[1]
        let comps = prepare_computations_for_refraction(&xs[1], &r, &xs);
        let c = refracted_color(&w, &comps, 5);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    // The refracted color with a refracted ray
    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = default_world();
        {
            let a = &mut w.objects[0];
            a.material.ambient = 1.0;
            a.material.set_pattern(&test_pattern());
        }
        {
            let b = &mut w.objects[1];
            b.material.transparency = 1.0;
            b.material.refractive_index = 1.5;
        }
        let a = &w.objects[0];
        let b = &w.objects[1];

        let r = ray(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let xs = intersections!(
            Intersection::new(-0.9899, Some(a)),
            Intersection::new(-0.4899, Some(b)),
            Intersection::new(0.4899, Some(b)),
            Intersection::new(0.9899, Some(a))
        );
        let comps = prepare_computations_for_refraction(&xs[2], &r, &xs);
        let c = refracted_color(&w, &comps, 5);
        assert_relative_eq!(c, color(0.0, 0.99888, 0.04725), epsilon = 1e-4);
    }

    // shade_hit() with a transparent material
    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = default_world();
        let mut floor = plane();
        floor.set_transform(&translation(0.0, -1.0, 0.0));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.add_object(floor);

        let mut ball = sphere(1);
        ball.material.color = color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transform(&translation(0.0, -3.5, -0.5));
        w.add_object(ball);

        let floor = match &w.objects[..] {
            [.., floor, _] => Some(floor),
            _ => panic!("missing objects"),
        };

        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -k, k));
        let xs = intersections!(Intersection::new(f64::sqrt(2.0), floor));
        let comps = prepare_computations_for_refraction(&xs[0], &r, &xs);
        let color_ = shade_hit(&w, &comps, 5);
        assert_relative_eq!(color_, color(0.93642, 0.68642, 0.68642), epsilon = 1e-4);
    }

    // shade_hit() with a reflective, transparent material
    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = default_world();
        let k = f64::sqrt(2.0) / 2.0;
        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -k, k));

        let mut floor = plane();
        floor.set_transform(&translation(0.0, -1.0, 0.0));
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.add_object(floor);

        let mut ball = sphere(1);
        ball.material.color = color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.set_transform(&translation(0.0, -3.5, -0.5));
        w.add_object(ball);

        let floor = match &w.objects[..] {
            [.., floor, _] => Some(floor),
            _ => panic!("missing objects"),
        };

        let xs = intersections!(Intersection::new(f64::sqrt(2.0), floor));
        let comps = prepare_computations_for_refraction(&xs[0], &r, &xs);
        let color_ = shade_hit(&w, &comps, 5);
        assert_relative_eq!(color_, color(0.93391, 0.69643, 0.69243), epsilon = 1e-5);
    }

    // Groups tests (Chapter 14)

    // Intersecting a ray with an empty group
    #[test]
    fn intersect_ray_with_empty_group() {
        let mut w = default_world();
        let g = group();
        let g_idx = w.add_object(g);
        let g = w.get_object_ref(&g_idx);
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = g.local_intersect(&r, Some(&w));
        assert!(xs.is_empty());
    }

    // Intersecting a ray with a non-empty group
    #[test]
    fn intersect_ray_with_non_empty_group() {
        let mut w = default_world();
        let g = group();
        let g_idx = w.add_object(g);
        let s1 = sphere(1);
        let s1_idx = w.add_object(s1);
        let mut s2 = sphere(2);
        s2.set_transform(&translation(0.0, 0.0, -3.0));
        let s2_idx = w.add_object(s2);
        let mut s3 = sphere(3);
        s3.set_transform(&translation(5.0, 0.0, 0.0));
        let s3_idx = w.add_object(s3);

        assert!(w.add_child(&g_idx, &s1_idx).is_ok());
        assert!(w.add_child(&g_idx, &s2_idx).is_ok());
        assert!(w.add_child(&g_idx, &s3_idx).is_ok());

        let g = w.get_object_ref(&g_idx);
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = g.local_intersect(&r, Some(&w));

        let s1 = w.get_object_ref(&s1_idx);
        let s2 = w.get_object_ref(&s2_idx);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, Some(s2));
        assert_eq!(xs[1].object, Some(s2));
        assert_eq!(xs[2].object, Some(s1));
        assert_eq!(xs[3].object, Some(s1));
    }

    // Intersecting a transformed group
    #[test]
    fn intersecting_a_transformed_group() {
        let mut w = default_world();
        let mut g = group();
        g.set_transform(&scaling(2.0, 2.0, 2.0));
        let g_idx = w.add_object(g);

        let mut s = sphere(1);
        s.set_transform(&translation(5.0, 0.0, 0.0));
        let s_idx = w.add_object(s);

        assert!(w.add_child(&g_idx, &s_idx).is_ok());

        let g = w.get_object_ref(&g_idx);
        let r = ray(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
        let xs = intersect(&g, &r, Some(&w));

        assert_eq!(xs.len(), 2);
    }
}
