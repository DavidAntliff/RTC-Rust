// Chapter 2: Drawing On a Canvas

use crate::colors::Color;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let capacity = usize::try_from(width * height).expect("Dimensions too large");
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); capacity],
        }
    }

    fn _index_of(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            usize::try_from(x + y * self.width).ok()
        } else {
            None
        }
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> &Color {
        match self._index_of(x, y) {
            Some(c) => &self.pixels[c],
            _ => panic!("panic"),
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: &Color) {
        let index = self
            ._index_of(x, y)
            .expect("Pixel coordinates out of range");
        self.pixels[index] = *color;
    }

    fn _add_value(row: &mut String, value: f64) {
        let v = f64::min(f64::max(value, 0.0), 1.0);
        let ivalue = (v * 255.0).round() as i32;
        if !row.is_empty() {
            row.push(' ');
        }
        row.push_str(&ivalue.to_string());
    }

    fn _split_line_by(line: &str, limit: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut view = line;
        while view.len() >= limit {
            let head = &view[0..limit - 1];
            let idx = head.rfind(' ').expect("No space character found");
            let left = &view[0..idx];
            lines.push(left.to_string());
            view = &view[idx + 1..];
        }
        lines.push(view.to_string());
        lines
    }

    pub fn to_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut data = String::new();

        for y in 0..self.height {
            let mut row = String::new();
            for x in 0..self.width {
                let p = self.pixel_at(x, y);
                Canvas::_add_value(&mut row, p.red());
                Canvas::_add_value(&mut row, p.green());
                Canvas::_add_value(&mut row, p.blue());
            }

            let lines = Canvas::_split_line_by(&row, 70);

            for line in lines {
                data.push_str(&line);
                data.push('\n');
            }
        }

        header + &data
    }
}

pub fn canvas(width: u32, height: u32) -> Canvas {
    Canvas::new(width, height)
}

pub fn pixel_at(c: &Canvas, x: u32, y: u32) -> &Color {
    c.pixel_at(x, y)
}

pub fn write_pixel(c: &mut Canvas, x: u32, y: u32, color: &Color) {
    c.write_pixel(x, y, color);
}

pub fn ppm_from_canvas(c: &Canvas) -> String {
    c.to_ppm()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::color;

    // Creating a canvas
    #[test]
    fn creating_a_canvas() {
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for x in 0..c.width {
            for y in 0..c.height {
                assert_eq!(*pixel_at(&c, x, y), color(0., 0., 0.));
            }
        }
    }

    // Writing pixels to a canvas
    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1., 0., 0.);
        write_pixel(&mut c, 2, 3, &red);
        assert_eq!(*pixel_at(&c, 2, 2), color(0., 0., 0.));
        assert_eq!(*pixel_at(&c, 2, 4), color(0., 0., 0.));
        assert_eq!(*pixel_at(&c, 2, 3), red);
    }

    // Constructing the PPM header
    #[test]
    fn construct_ppm_header() {
        let c = canvas(5, 3);
        let ppm = ppm_from_canvas(&c);
        assert!(ppm.starts_with("P3\n5 3\n255\n"));
    }

    // Constructing the PPM pixel data
    #[test]
    fn construct_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);
        write_pixel(&mut c, 0, 0, &c1);
        write_pixel(&mut c, 2, 1, &c2);
        write_pixel(&mut c, 4, 2, &c3);
        let ppm = ppm_from_canvas(&c);

        let lines = ppm.split('\n').collect::<Vec<_>>();

        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    // Splitting long lines in PPM files
    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = canvas(10, 2);
        for y in 0..c.height {
            for x in 0..c.width {
                write_pixel(&mut c, x, y, &color(1.0, 0.8, 0.6));
            }
        }
        let ppm = ppm_from_canvas(&c);

        let lines = ppm.split('\n').collect::<Vec<_>>();

        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    // PPM files are terminated by a newline character
    #[test]
    fn ppm_terminated_by_newline() {
        let c = canvas(5, 3);
        let ppm = ppm_from_canvas(&c);

        assert!(ppm.ends_with('\n'));
    }
}
