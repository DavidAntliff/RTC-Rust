use crate::colors::Color;
use crate::json5::{
    load_scene, BodyType, Light, LightType, Material as JsonMaterial, Scene, Transform,
};
use crate::lights::point_light;
use crate::materials::{default_material, Material};
use crate::matrices::identity4;
use crate::matrices::Matrix4;
use crate::shapes::{plane, sphere};
use crate::transformations::{rotation_x, rotation_y, rotation_z, scaling, translation};
use crate::tuples::{point, Tuple};
use crate::world::{world, World};
use anyhow::{anyhow, Result};
use std::path::Path;

impl From<[f64; 3]> for Color {
    fn from(value: [f64; 3]) -> Self {
        Color::new(value[0], value[1], value[2])
    }
}

impl From<[f64; 3]> for Tuple {
    fn from(value: [f64; 3]) -> Self {
        point(value[0], value[1], value[2])
    }
}

fn build_transform(transforms: &Option<Vec<Transform>>) -> Matrix4 {
    let mut combined_transform = identity4();
    if let Some(transforms) = transforms {
        for transform in transforms {
            let t = match transform {
                Transform::RotateX(theta) => rotation_x(*theta),
                Transform::RotateY(theta) => rotation_y(*theta),
                Transform::RotateZ(theta) => rotation_z(*theta),
                Transform::Translate(x, y, z) => translation(*x, *y, *z),
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
    m
}

pub fn load_world(filename: &Path) -> Result<World> {
    let mut world = world();
    let scene = load_scene(filename)?;

    if let Some(lights) = scene.lights {
        for light in lights {
            match light.light_type {
                LightType::PointLight => {
                    let l = point_light(light.position.into(), light.intensity.into());
                    world.add_light(l);
                }
                LightType::SpotLight => {
                    todo!()
                }
            }
        }
    }

    if let Some(bodies) = scene.bodies {
        for body in bodies {
            let shape = match body.body_type {
                BodyType::Plane => {
                    let mut shape = plane();
                    shape.set_transform(&build_transform(&body.transforms));
                    shape.material = build_material(&body.material);
                    shape
                }
                BodyType::Sphere => {
                    let mut shape = sphere(1);
                    shape.set_transform(&build_transform(&body.transforms));
                    shape.material = build_material(&body.material);
                    shape
                }
            };

            world.add_object(shape);
        }
    }

    Ok(world)
}
