// Chapter 7: Implementing a Camera

use crate::canvas::{canvas, Canvas};
use crate::matrices::{identity4, Matrix4};
use crate::rays::{ray, Ray};
use crate::tuples::{normalize, point};
use crate::world::{color_at, World};
use std::f64::consts::PI;

pub struct Camera {
    hsize: u32,
    vsize: u32,
    #[allow(dead_code)]
    field_of_view: f64,
    transform: Matrix4,

    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
        let c = calc_pixel_size(hsize, vsize, field_of_view);
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: identity4(),
            half_width: c.half_width,
            half_height: c.half_height,
            pixel_size: c.pixel_size,
        }
    }

    pub fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = *transform;
    }

    pub fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (the camera looks toward -Z, so +X is to the *left*)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (the canvas is at Z=-1)
        let inverse_camera_transform = self.transform.inverse();
        let pixel = inverse_camera_transform * point(world_x, world_y, -1.0);
        let origin = inverse_camera_transform * point(0.0, 0.0, 0.0);
        let direction = normalize(&(pixel - origin));

        ray(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = canvas(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = ray_for_pixel(self, x, y);
                let color = color_at(world, &ray);
                image.write_pixel(x, y, &color);
            }
        }
        image
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(100, 50, PI / 3.0)
    }
}

pub fn camera(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
    Camera::new(hsize, vsize, field_of_view)
}

pub fn ray_for_pixel(camera: &Camera, px: u32, py: u32) -> Ray {
    camera.ray_for_pixel(px, py)
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    camera.render(world)
}

struct CalcPixelSizeResult {
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

fn calc_pixel_size(hsize: u32, vsize: u32, field_of_view: f64) -> CalcPixelSizeResult {
    let half_view = f64::tan(field_of_view / 2.0);
    let aspect_ratio = hsize as f64 / vsize as f64;
    let (half_width, half_height) = if aspect_ratio >= 1.0 {
        (half_view, half_view / aspect_ratio)
    } else {
        (half_view * aspect_ratio, half_view)
    };
    CalcPixelSizeResult {
        half_width,
        half_height,
        pixel_size: half_width * 2.0 / hsize as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::color;
    use crate::matrices::identity4;
    use crate::transformations::{rotation_y, translation, view_transform};
    use crate::tuples::vector;
    use crate::world::default_world;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    // Constructing a camera
    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = camera(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_relative_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, identity4());
    }

    // The pixel size for a horizontal canvas
    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = camera(200, 125, PI / 2.0);
        assert_relative_eq!(c.pixel_size, 0.01);
    }

    // The pixel size for a vertical canvas
    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = camera(125, 200, PI / 2.0);
        assert_relative_eq!(c.pixel_size, 0.01);
    }

    // Constructing a ray through the center of the canvas
    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = camera(201, 101, PI / 2.0);
        let r = ray_for_pixel(&c, 100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_relative_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    // Constructing a ray through the corner of the canvas
    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = camera(201, 101, PI / 2.0);
        let r = ray_for_pixel(&c, 0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_relative_eq!(
            r.direction,
            vector(0.66519, 0.33259, -0.66851),
            epsilon = 1e-5
        );
    }

    // Constructing a ray when the camera is transformed
    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let mut c = camera(201, 101, PI / 2.0);
        c.set_transform(&(rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0)));
        let r = ray_for_pixel(&c, 100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        let k = f64::sqrt(2.0) / 2.0;
        assert_relative_eq!(r.direction, vector(k, 0.0, -k));
    }

    // Rendering a world with a camera
    #[test]
    fn rendering_world_with_camera() {
        let w = default_world();
        let mut c = camera(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.set_transform(&view_transform(&from, &to, &up));
        let image = render(&c, &w);
        assert_relative_eq!(
            image.pixel_at(5, 5),
            &color(0.38066, 0.47583, 0.2855),
            epsilon = 1e-5
        );
    }
}
