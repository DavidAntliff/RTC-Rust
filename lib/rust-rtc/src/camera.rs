// Chapter 7: Implementing a Camera

use crate::canvas::{canvas, Canvas};
use crate::matrices::{identity4, Matrix4};
use crate::rays::{ray, Ray};
use crate::tuples::{normalize, point};
use crate::world::{color_at, World};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
//use std::time::Instant;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Resolution {
    pub hsize: u32, // pixel width
    pub vsize: u32, // pixel height
}

impl Resolution {
    pub const fn new(hsize: u32, vsize: u32) -> Self {
        Resolution { hsize, vsize }
    }

    pub fn num_pixels(&self) -> u64 {
        self.hsize as u64 * self.vsize as u64
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            hsize: 100,
            vsize: 50,
        }
    }
}

impl Resolution {
    pub const VGA: Resolution = Resolution::new(640, 480);
    pub const SVGA: Resolution = Resolution::new(800, 600);
    pub const XGA: Resolution = Resolution::new(1024, 768);
    pub const SXGA: Resolution = Resolution::new(1280, 1024);
    pub const FHD: Resolution = Resolution::new(1920, 1080);
    pub const QHD: Resolution = Resolution::new(2560, 1440);
    pub const UHD_4K: Resolution = Resolution::new(3840, 2160);
}

pub struct Camera {
    resolution: Resolution,

    #[allow(dead_code)]
    field_of_view: f64,  // stored, but not used

    transform: Matrix4,
    inverse_transform: Matrix4,

    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(resolution: Resolution, field_of_view: f64) -> Camera {
        let c = calc_pixel_size(resolution.hsize, resolution.vsize, field_of_view);
        Camera {
            resolution,
            field_of_view,
            half_width: c.half_width,
            half_height: c.half_height,
            pixel_size: c.pixel_size,
            ..Default::default()
        }
    }

    pub fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = *transform;
        self.inverse_transform = self.transform.inverse();
    }

    pub fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    pub fn inverse_transform(&self) -> &Matrix4 {
        &self.inverse_transform
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
        let pixel = self.inverse_transform * point(world_x, world_y, -1.0);
        let origin = self.inverse_transform * point(0.0, 0.0, 0.0);
        let direction = normalize(&(pixel - origin));

        ray(origin, direction)
    }

    pub fn render_single_threaded(&self, world: &World, max_recursive_depth: i32,
                                  mut progress_callback: Option<Box<dyn FnMut(u64) + '_>>) -> Canvas {
        let mut image = canvas(self.resolution.hsize, self.resolution.vsize);

        for y in 0..self.resolution.vsize {
            for x in 0..self.resolution.hsize {
                let ray = ray_for_pixel(self, x, y);
                let color = color_at(world, &ray, max_recursive_depth);
                image.write_pixel(x, y, &color);
            }

            match &mut progress_callback {
                Some(f) => (f)(self.resolution.hsize as u64),
                None => (),
            };
        }
        image
    }

    // https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust
    pub fn render(&self, world: &World, max_recursive_depth: i32,
                  xdiv: u32, ydiv: u32,
                  progress_callback: Option<Box<dyn FnMut(u64) + Send + '_>>) -> Canvas {
        let image = canvas(self.resolution.hsize, self.resolution.vsize);

        let image_height = image.height;
        let image_width = image.width;

        let ystep = image_height / ydiv;
        let xstep = image_width / xdiv;

        let image_arc = Arc::new(Mutex::new(image));
        let pb_arc = progress_callback.map(|x| Arc::new(Mutex::new(x)));

        std::thread::scope(|s| {
            for y in 0..ydiv {
                for x in 0..xdiv {
                    let image = Arc::clone(&image_arc);
                    let pb_opt = pb_arc.as_ref().map(|x| Arc::clone(&x));

                    s.spawn(move || {
                        //eprintln!("thread {}, {} started", x, y);
                        //let now = Instant::now();

                        // Account for rounding loss due to integer division in the bottom/right subimages:
                        let xstart = x * xstep;
                        let xend = if x == xdiv - 1 { image_width } else { (x + 1) * xstep };
                        let ystart = y * ystep;
                        let yend = if y == ydiv - 1 { image_height } else { (y + 1) * ystep };

                        let subimage = self.render_subimage(world,
                                                            xstart, xend,
                                                            ystart, yend,
                                                            max_recursive_depth,
                                                            pb_opt);

                        //eprintln!("thread {:2}, {:2} finished in {:6} ms", x, y, now.elapsed().as_millis());

                        let mut image = image.lock().expect("should be lockable");
                        image.blit(&subimage, x * xstep, y * ystep);
                    });
                }
            }
        });

        Arc::try_unwrap(image_arc).expect("should be sole owner").into_inner().expect("should be consumable")
    }

    pub fn render_subimage(&self, world: &World,
                           start_x: u32, end_x: u32,
                           start_y: u32, end_y: u32,
                           max_recursive_depth: i32,
                           progress_callback: Option<Arc<Mutex<Box<dyn FnMut(u64) + Send + '_>>>>) -> Canvas {
        let height = end_y - start_y;
        let width = end_x - start_x;
        let mut image = canvas(width, height);

        for y in 0..height {
            for x in 0..width {
                let ray = ray_for_pixel(self, start_x + x, start_y + y);
                let color = color_at(world, &ray, max_recursive_depth);
                image.write_pixel(x, y, &color);
            }

            {
                // unstable: if pb_opt.is_some_and(|x| ...)
                match &progress_callback {
                    Some(ref arc) => {
                        let mut f = arc.lock().expect("should be lockable");
                        (f)(width as u64);
                    },
                    None => (),
                }
            }
        }
        image
    }
}

impl Default for Camera {
    fn default() -> Camera {
        let default_resolution = Resolution::default();
        let default_field_of_view = PI / 3.0;
        let c = calc_pixel_size(
            default_resolution.hsize,
            default_resolution.vsize,
            default_field_of_view,
        );

        Camera {
            resolution: default_resolution,
            field_of_view: default_field_of_view,
            transform: identity4(),
            inverse_transform: identity4(),
            half_width: c.half_width,
            half_height: c.half_height,
            pixel_size: c.pixel_size,
        }
    }
}

pub fn camera(resolution: Resolution, field_of_view: f64) -> Camera {
    Camera::new(resolution, field_of_view)
}

pub fn ray_for_pixel(camera: &Camera, px: u32, py: u32) -> Ray {
    camera.ray_for_pixel(px, py)
}

pub fn render(camera: &mut Camera, world: &World, max_recursive_depth: i32) -> Canvas {
    camera.render_single_threaded(world, max_recursive_depth, None)
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
        let c = camera(Resolution::new(hsize, vsize), field_of_view);
        assert_eq!(c.resolution.hsize, 160);
        assert_eq!(c.resolution.vsize, 120);
        assert_relative_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, identity4());
    }

    // The pixel size for a horizontal canvas
    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = camera(Resolution::new(200, 125), PI / 2.0);
        assert_relative_eq!(c.pixel_size, 0.01);
    }

    // The pixel size for a vertical canvas
    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = camera(Resolution::new(125, 200), PI / 2.0);
        assert_relative_eq!(c.pixel_size, 0.01);
    }

    // Constructing a ray through the center of the canvas
    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = camera(Resolution::new(201, 101), PI / 2.0);
        let r = ray_for_pixel(&c, 100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_relative_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    // Constructing a ray through the corner of the canvas
    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = camera(Resolution::new(201, 101), PI / 2.0);
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
        let mut c = camera(Resolution::new(201, 101), PI / 2.0);
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
        let mut c = camera(Resolution::new(11, 11), PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.set_transform(&view_transform(&from, &to, &up));
        let image = render(&mut c, &w, 1);
        assert_relative_eq!(
            image.pixel_at(5, 5),
            &color(0.38066, 0.47583, 0.2855),
            epsilon = 1e-5
        );
    }
}
