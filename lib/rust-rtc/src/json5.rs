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
    pub(crate) cameras: Option<Vec<Camera>>,
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

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub(crate) struct Camera {
    pub(crate) name: String,
    pub(crate) resolution: Resolution,
    pub(crate) field_of_view: f64,
    pub(crate) from: [f64; 3],
    pub(crate) to: [f64; 3],
    pub(crate) up: [f64; 3],
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            resolution: Resolution::default(),
            field_of_view: 0.0,
            from: [0.0, 0.0, -10.0],
            to: [0.0, 1.0, 0.0],
            up: [0.0, 1.0, 0.0],
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) enum Resolution {
    VGA,
    SVGA,
    XGA,
    XSGA,
    FHD,
    QHD,
    #[serde(rename = "UHD")]
    UHD,
    #[serde(untagged)]
    Custom {
        width: u32,
        height: u32,
    },
}

impl Default for Resolution {
    fn default() -> Self {
        Self::Custom {
            width: 100,
            height: 50,
        }
    }
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
