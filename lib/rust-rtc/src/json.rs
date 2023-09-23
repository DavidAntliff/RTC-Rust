use crate::materials::RefractiveIndex;
use anyhow::Result;
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
pub(crate) enum Light {
    #[serde(rename = "point_light")]
    PointLight {
        position: [f64; 3],
        intensity: [f64; 3],
    },
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct BodyCommon {
    pub(crate) material: Option<Material>,
    pub(crate) transforms: Option<Vec<Transform>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Plane {
    #[serde(flatten)]
    pub(crate) common: BodyCommon,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Sphere {
    #[serde(flatten)]
    pub(crate) common: BodyCommon,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Cone {
    #[serde(flatten)]
    pub(crate) common: BodyCommon,
    pub(crate) minimum_y: Option<f64>,
    pub(crate) maximum_y: Option<f64>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Cylinder {
    #[serde(flatten)]
    pub(crate) common: BodyCommon,
    pub(crate) minimum_y: Option<f64>,
    pub(crate) maximum_y: Option<f64>,
    pub(crate) closed_min: Option<bool>,
    pub(crate) closed_max: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct Cube {
    #[serde(flatten)]
    pub(crate) common: BodyCommon,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) enum Body {
    #[serde(rename = "plane")]
    Plane(Plane),
    #[serde(rename = "sphere")]
    Sphere(Sphere),
    #[serde(rename = "cone")]
    Cone(Cone),
    #[serde(rename = "cylinder")]
    Cylinder(Cylinder),
    #[serde(rename = "cube")]
    Cube(Cube),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub(crate) struct Material {
    pub(crate) color: Color,
    pub(crate) ambient: f64,
    pub(crate) diffuse: f64,
    pub(crate) specular: f64,
    pub(crate) shininess: f64,
    pub(crate) reflective: f64,
    pub(crate) transparency: f64,
    pub(crate) refractive_index: f64,
    pub(crate) casts_shadow: bool,
    pub(crate) receives_shadow: bool,
    pub(crate) pattern: Option<Pattern>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(untagged)]
pub(crate) enum Color {
    #[serde(rename = "color")]
    Color([f64; 3]),
    #[serde(rename = "colori")]
    Colori([i32; 3]),
}

// TODO: take these defaults from materials.rs
impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::Color([1.0, 1.0, 1.0]),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: RefractiveIndex::AIR,
            casts_shadow: true,
            receives_shadow: true,
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
    #[serde(rename = "translate_x")]
    TranslateX(f64),
    #[serde(rename = "translate_y")]
    TranslateY(f64),
    #[serde(rename = "translate_z")]
    TranslateZ(f64),
    #[serde(rename = "scale")]
    Scale(f64, f64, f64),
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) enum Pattern {
    #[serde(rename = "color")]
    Color(f64, f64, f64),
    #[serde(rename = "colori")]
    Colori(i32, i32, i32),
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
    #[serde(rename = "checkers")]
    Checkers {
        a: Box<Pattern>,
        b: Box<Pattern>,
        transforms: Option<Vec<Transform>>,
    },
    #[serde(rename = "stripes")]
    Stripes {
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
    pub(crate) resolution: Option<Resolution>,
    pub(crate) field_of_view: Option<f64>,
    pub(crate) from: [f64; 3],
    pub(crate) to: [f64; 3],
    pub(crate) up: [f64; 3],
    pub(crate) transforms: Option<Vec<Transform>>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            name: "main".to_string(),
            resolution: None,
            field_of_view: None,
            from: [0.0, 0.0, -10.0],
            to: [0.0, 1.0, 0.0],
            up: [0.0, 1.0, 0.0],
            transforms: None,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
#[serde(deny_unknown_fields)]
pub(crate) enum Resolution {
    VGA,
    SVGA,
    XGA,
    SXGA,
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

    //let deserializer = &mut json5::from_str(&data);
    //let deserializer = json5::Deserializer::from_str(&data);

    //let result: Result<T, _> = serde_path_to_error::deserialize(deserializer);

    Ok(t)
    //result

    //let jd = &mut json5::Deserializer::from_str(&data);
    //let result: Result<T, _> = serde_path_to_error::deserialize(jd);

    //Ok(())
}

pub fn load_scene(filename: &Path) -> Result<Scene> {
    load_json5::<Scene>(filename)
}
