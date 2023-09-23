use crate::camera::Resolution;
use crate::colors::{color, colori, Color};
use crate::json5::{
    load_scene, Body, Color as JsonColor, Light, Material as JsonMaterial, Pattern as JsonPattern,
    Resolution as JsonResolution, Transform,
};
use crate::lights::point_light;
use crate::materials::{default_material, Material};
use crate::matrices::identity4;
use crate::matrices::Matrix4;
use crate::patterns::{
    checkers_pattern, radial_gradient_pattern, ring_pattern, solid_pattern, stripe_pattern, Pattern,
};
use crate::transformations::{
    rotation_x, rotation_y, rotation_z, scaling, translate_x, translate_y, translate_z,
    translation, view_transform,
};
use crate::tuples::{point, Tuple};
use crate::utils::RenderOptions;
use crate::world::{world, World};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

impl From<[f64; 3]> for Color {
    fn from(value: [f64; 3]) -> Self {
        Color::new(value[0], value[1], value[2])
    }
}

impl From<[i32; 3]> for Color {
    fn from(value: [i32; 3]) -> Self {
        colori(value[0], value[1], value[2])
    }
}

impl From<[f64; 3]> for Tuple {
    fn from(value: [f64; 3]) -> Self {
        point(value[0], value[1], value[2])
    }
}

impl From<JsonColor> for Color {
    fn from(value: crate::json5::Color) -> Self {
        match value {
            JsonColor::Color(ar) => ar.into(),
            JsonColor::Colori(ar) => ar.into(),
        }
    }
}

impl From<JsonResolution> for Resolution {
    fn from(value: JsonResolution) -> Self {
        match value {
            JsonResolution::VGA => Resolution::VGA,
            JsonResolution::SVGA => Resolution::SVGA,
            JsonResolution::XGA => Resolution::XGA,
            JsonResolution::SXGA => Resolution::SXGA,
            JsonResolution::FHD => Resolution::FHD,
            JsonResolution::QHD => Resolution::QHD,
            JsonResolution::UHD => Resolution::UHD_4K,
            JsonResolution::Custom { width, height } => Resolution::new(width, height),
        }
    }
}

fn build_transform(initial: &Matrix4, transforms: &Option<Vec<Transform>>) -> Matrix4 {
    let mut combined_transform = *initial;
    if let Some(transforms) = transforms {
        for transform in transforms {
            let t = match transform {
                Transform::RotateX(theta) => rotation_x(*theta),
                Transform::RotateY(theta) => rotation_y(*theta),
                Transform::RotateZ(theta) => rotation_z(*theta),
                Transform::Translate(x, y, z) => translation(*x, *y, *z),
                Transform::TranslateX(x) => translate_x(*x),
                Transform::TranslateY(y) => translate_y(*y),
                Transform::TranslateZ(z) => translate_z(*z),
                Transform::Scale(x, y, z) => scaling(*x, *y, *z),
            };
            combined_transform.then(&t);
        }
    }
    combined_transform
}

fn build_material(material: &JsonMaterial) -> Material {
    let mut m = default_material();
    m.color = material.color.into();
    m.ambient = material.ambient;
    m.diffuse = material.diffuse;
    m.specular = material.specular;
    m.shininess = material.shininess;
    m.reflective = material.reflective;
    m.transparency = material.transparency;
    m.refractive_index = material.refractive_index;
    m.casts_shadow = material.casts_shadow;
    m.receives_shadow = material.receives_shadow;

    if let Some(base_pattern) = &material.pattern {
        m.set_pattern(&build_pattern(&base_pattern));
    }

    m
}

fn build_pattern(pattern: &JsonPattern) -> Pattern {
    match pattern {
        JsonPattern::Color(r, g, b) => solid_pattern(&color(*r, *g, *b)),
        JsonPattern::Colori(r, g, b) => solid_pattern(&colori(*r, *g, *b)),
        JsonPattern::RadialGradient {
            a,
            b,
            transforms,
            y_factor,
        } => {
            let mut p = radial_gradient_pattern(build_pattern(a), build_pattern(b), *y_factor);
            p.set_transform(&build_transform(&identity4(), transforms));
            p
        }
        JsonPattern::Rings { a, b, transforms } => {
            let mut p = ring_pattern(build_pattern(a), build_pattern(b));
            p.set_transform(&build_transform(&identity4(), transforms));
            p
        }
        JsonPattern::Checkers { a, b, transforms } => {
            let mut p = checkers_pattern(build_pattern(a), build_pattern(b));
            p.set_transform(&build_transform(&identity4(), transforms));
            p
        }
        JsonPattern::Stripes { a, b, transforms } => {
            let mut p = stripe_pattern(build_pattern(a), build_pattern(b));
            p.set_transform(&build_transform(&identity4(), transforms));
            p
        }
    }
}

pub fn load_world(filename: &Path) -> Result<(World, HashMap<String, RenderOptions>)> {
    let mut world = world();
    let scene = load_scene(filename)?;

    if let Some(lights) = scene.lights {
        for light in lights {
            match light {
                Light::PointLight {
                    position,
                    intensity,
                } => {
                    let l = point_light(position.into(), intensity.into());
                    world.add_light(l);
                }
            }
        }
    }

    if let Some(bodies) = scene.bodies {
        for body in bodies {
            let shape = match body {
                Body::Plane(plane) => {
                    let mut shape = crate::shapes::plane();
                    shape.set_transform(&build_transform(&identity4(), &plane.common.transforms));
                    if let Some(m) = plane.common.material {
                        shape.material = build_material(&m);
                    };
                    shape
                }
                Body::Sphere(sphere) => {
                    let mut shape = crate::shapes::sphere(1);
                    shape.set_transform(&build_transform(&identity4(), &sphere.common.transforms));
                    if let Some(m) = sphere.common.material {
                        shape.material = build_material(&m);
                    };
                    shape
                }
                Body::Cone(cone) => {
                    let mut shape = crate::shapes::cone();
                    let p = shape.as_cone_primitive().expect("should be a cone");
                    if let Some(minimum_y) = cone.minimum_y {
                        p.minimum_y = minimum_y;
                    }
                    if let Some(maximum_y) = cone.maximum_y {
                        p.maximum_y = maximum_y;
                    }
                    shape.set_transform(&build_transform(&identity4(), &cone.common.transforms));
                    if let Some(m) = cone.common.material {
                        shape.material = build_material(&m);
                    };
                    shape
                }
                Body::Cylinder(cylinder) => {
                    let min_y = cylinder.minimum_y.unwrap_or(-1.0);
                    let max_y = cylinder.maximum_y.unwrap_or(1.0);
                    let closed_min = cylinder.closed_min.unwrap_or(true);
                    let closed_max = cylinder.closed_max.unwrap_or(true);

                    let mut shape = crate::shapes::cylinder(min_y, max_y, closed_min, closed_max);
                    shape
                        .set_transform(&build_transform(&identity4(), &cylinder.common.transforms));
                    if let Some(m) = cylinder.common.material {
                        shape.material = build_material(&m);
                    };
                    shape
                }
                Body::Cube(cube) => {
                    let mut shape = crate::shapes::cube();
                    shape.set_transform(&build_transform(&identity4(), &cube.common.transforms));
                    if let Some(m) = cube.common.material {
                        shape.material = build_material(&m);
                    };
                    shape
                }
            };
            world.add_object(shape);
        }
    }

    let mut coll = HashMap::<String, RenderOptions>::new();
    if let Some(cameras) = scene.cameras {
        for camera in cameras {
            // 'up' must be a vector, so zero the w element:
            let mut up = Tuple::from(camera.up);
            up.set_w(0.0);

            let camera_transform = view_transform(&camera.from.into(), &camera.to.into(), &up);
            let camera_transform = build_transform(&camera_transform, &camera.transforms);

            let mut render_options = RenderOptions {
                camera_transform,
                ..Default::default()
            };
            if let Some(resolution) = camera.resolution {
                render_options.default_resolution = resolution.into();
            }
            if let Some(fov) = camera.field_of_view {
                render_options.field_of_view = fov;
            }

            coll.insert(camera.name, render_options);
        }
    }

    Ok((world, coll))
}
