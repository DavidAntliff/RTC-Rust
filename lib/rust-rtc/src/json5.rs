use anyhow::Result;
use json5;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub(crate) lights: Option<Vec<Light>>,
    pub(crate) bodies: Option<Vec<Body>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Light {
    #[serde(rename = "type")]
    pub(crate) light_type: LightType,
    pub(crate) position: [f64; 3],
    pub(crate) intensity: [f64; 3],
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) enum LightType {
    #[serde(rename = "point_light")]
    PointLight,
    #[serde(rename = "spot_light")]
    SpotLight,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Body {
    #[serde(rename = "type")]
    pub(crate) body_type: BodyType,
    pub(crate) material: Material,
    pub(crate) transforms: Option<Vec<Transform>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) enum BodyType {
    #[serde(rename = "plane")]
    Plane,
    #[serde(rename = "sphere")]
    Sphere,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub(crate) struct Material {
    pub(crate) color: [f64; 3],
    pub(crate) ambient: f64,
    pub(crate) diffuse: f64,
    pub(crate) specular: f64,
    pub(crate) pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            specular: 0.9,
            ambient: 0.1,
            diffuse: 0.9,
            pattern: None,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) enum Transform {
    #[serde(rename = "rotate_x")]
    RotateX(f64),
    #[serde(rename = "rotate_y")]
    RotateY(f64),
    #[serde(rename = "rotate_z")]
    RotateZ(f64),
    #[serde(rename = "translate")]
    Translate(f64, f64, f64),
    #[serde(rename = "scale")]
    Scale(f64, f64, f64),
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) enum Pattern {
    #[serde(rename = "color")]
    Color(f64, f64, f64),
    #[serde(rename = "radial_gradient")]
    RadialGradient {
        a: Box<Pattern>,
        b: Box<Pattern>,
        transforms: Option<Vec<Transform>>,
        y_factor: f64,
    },
    #[serde(rename = "rings")]
    Rings {
        a: Box<Pattern>,
        b: Box<Pattern>,
        transforms: Option<Vec<Transform>>,
    },
}

fn load_json5<T>(filename: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let data = std::fs::read_to_string(filename)?;
    let t: T = json5::from_str(&data)?;
    Ok(t)
}

pub fn load_scene(filename: &Path) -> Result<Scene> {
    load_json5::<Scene>(filename)
}
