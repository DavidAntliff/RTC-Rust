// Chapter 6: Lights and Shading

use crate::colors::Color;
use crate::tuples::Point;

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

pub fn point_light(position: Point, intensity: Color) -> PointLight {
    PointLight::new(position, intensity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::color;
    use crate::tuples::point;

    // A point light has a position and intensity
    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = color(1.0, 1.0, 1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
