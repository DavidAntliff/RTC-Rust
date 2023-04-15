// Chapter 6: Lights and Shading

use crate::colors::{color, Color};
use crate::lights::PointLight;
use crate::spheres::Sphere;
use crate::tuples::{dot, normalize, reflect, Point, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    //TODO pattern: &Pattern,
}

impl Material {
    pub fn new(color: Color,
               ambient: f64,
               diffuse: f64,
               specular: f64,
               shininess: f64) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn lighting(
        &self,
        _object: &Sphere,
        light: &Option<PointLight>,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        // TODO
        //let pattern = material.pattern;
        //let material_color = pattern ? pattern_at_shape(*pattern, shape, point) : material.color()};
        let material_color = self.color;

        // Light is optional
        let light_intensity: Color;
        let light_position: Point;
        if let Some(light) = light {
            light_intensity = light.intensity;
            light_position = light.position;
        } else {
            light_intensity = color(0.0, 0.0, 0.0);
            light_position = crate::tuples::point(0.0, 0.0, 0.0);
        }

        // Combine the surface color with the light's color/intensity
        let effective_color = material_color * light_intensity;

        // Find the direction to the light source
        let lightv = normalize(&(light_position - point));

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

        let diffuse: Color;
        let specular: Color;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = dot(&lightv, normalv);
        if light_dot_normal < 0.0 {
            diffuse = color(0.0, 0.0, 0.0); // black
            specular = color(0.0, 0.0, 0.0); // black
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = reflect(&(-lightv), normalv);
            let reflect_dot_eye = dot(&reflectv, eyev);

            if reflect_dot_eye <= 0.0 {
                specular = color(0.0, 0.0, 0.0);
            } else {
                // Compute the specular contribution
                let factor = f64::powf(reflect_dot_eye, self.shininess);
                specular = light_intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub fn default_material() -> Material {
    Material::default()
}

pub fn material(color: Color,
                ambient: f64,
                diffuse: f64,
                specular: f64,
                shininess: f64) -> Material {
    Material::new(color, ambient, diffuse, specular, shininess)
}

pub fn lighting(
    material: &Material,
    object: &Sphere,
    light: &Option<PointLight>,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
    in_shadow: bool,
) -> Color {
    material.lighting(object, light, point, eyev, normalv, in_shadow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lights::point_light;
    use crate::spheres::sphere;
    use crate::tuples::{point, vector, Point};
    use approx::assert_relative_eq;
    use rstest::{fixture, rstest};

    // The default material
    #[test]
    fn the_default_material() {
        let m = default_material();
        assert_eq!(m.color, color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    struct MaterialFixture {
        m: Material,
        position: Point,
    }

    #[fixture]
    fn fix() -> MaterialFixture {
        MaterialFixture {
            m: default_material(),
            position: point(0.0, 0.0, 0.0),
        }
    }

    // Lighting with the eye between the light and the surface
    #[rstest]
    fn lighting_with_eye_between_light_and_surface(fix: MaterialFixture) {
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);

        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let result = lighting(
            &fix.m,
            &sphere(1),
            &Some(light),
            &fix.position,
            &eyev,
            &normalv,
            false,
        );

        // intensity = full ambient + full diffuse + full specular
        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    // Lighting with the eye between light and surface, eye offset 45 degrees
    #[rstest]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees(fix: MaterialFixture) {
        let k = f64::sqrt(2.0) / 2.0;
        let eyev = vector(0.0, k, -k);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let result = lighting(
            &fix.m,
            &sphere(1),
            &Some(light),
            &fix.position,
            &eyev,
            &normalv,
            false,
        );

        // intensity = full ambient + full diffuse + zero specular
        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    // Lighting with eye opposite surface, light offset 45 degrees
    #[rstest]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees(fix: MaterialFixture) {
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        let result = lighting(
            &fix.m,
            &sphere(1),
            &Some(light),
            &fix.position,
            &eyev,
            &normalv,
            false,
        );

        // intensity = full ambient + partial diffuse + zero specular
        assert_relative_eq!(result, color(0.7364, 0.7364, 0.7364), epsilon = 1e-4);
    }

    // Lighting with eye in the path of the reflection vector
    #[rstest]
    fn lighting_with_eye_in_path_of_reflection_vector(fix: MaterialFixture) {
        let k = f64::sqrt(2.0) / 2.0;
        let eyev = vector(0.0, -k, -k);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        let result = lighting(
            &fix.m,
            &sphere(1),
            &Some(light),
            &fix.position,
            &eyev,
            &normalv,
            false,
        );

        // intensity = full ambient + partial diffuse + full specular
        assert_relative_eq!(result, color(1.6364, 1.6364, 1.6364), epsilon = 1e-4);
    }

    // Lighting with the light behind the surface
    #[rstest]
    fn lighting_with_light_behind_surface(fix: MaterialFixture) {
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));
        let result = lighting(
            &fix.m,
            &sphere(1),
            &Some(light),
            &fix.position,
            &eyev,
            &normalv,
            false,
        );

        // intensity = full ambient + zero diffuse + zero specular
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    // Chapter 8: Shadows

    // Lighting with the surface in shadow
    #[rstest]
    fn lighting_with_surface_in_shadow(fix: MaterialFixture) {
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = lighting(
            &fix.m,
            &sphere(1),
            &Some(light),
            &fix.position,
            &eyev,
            &normalv,
            in_shadow,
        );

        assert_eq!(result, color(0.1, 0.1, 0.1));
    }
    /*
       TODO

       // Chapter 10: Patterns

       // Lighting with a pattern applied
       #[rstest]
       fn lighting_with_pattern_applied(fix: MaterialFixture) {
           m.set_pattern(stripe_pattern(color(1.0, 1.0, 1.0), color(0.0, 0.0, 0.0)));
           m.set_ambient(1.0);
           m.set_diffuse(0.0);
           m.set_specular(0.0);
           let eyev = vector(0.0, 0.0, -1.0);
           let normalv = vector(0.0, 0.0, -1.0);
           let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
           let c1 = lighting(m, sphere(1), light, point(0.9, 0.0, 0.0), eyev, normalv, false);
           let c2 = lighting(m, sphere(1), light, point(1.1, 0.0, 0.0), eyev, normalv, false);
           assert_eq!(c1, color(1.0, 1.0, 1.0));
           assert_eq!(c2, color(0.0, 0.0, 0.0));
       }

    */
}
