use super::tuples::{Tuple};

type Color = Tuple;

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Tuple::new(r, g, b, 0.0)
}

/// Hadamard or Shur Product
pub fn hadamard(lhs: &Color, rhs: &Color) -> Color {
    lhs * rhs
}

impl Color {
    pub fn red(&self) -> f64 { self.x() }
    pub fn green(&self) -> f64 { self.y() }
    pub fn blue(&self) -> f64 { self.z() }
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // Colors are (red, green, blue) tuples
    #[test]
    fn colors_are_tuples () {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    // Adding colors
    #[test]
    fn adding_colors () {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    // Subtracting colors
    #[test]
    fn subtracting_colors () {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_relative_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }

    // Multiplying a color by a scalar
    #[test]
    fn multiplying_color_by_scalar () {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    // Multiplying colors
    #[test]
    fn multiplying_colors () {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_relative_eq!(&c1 * &c2, color(0.9, 0.2, 0.04));
        assert_relative_eq!(hadamard(&c1, &c2), color(0.9, 0.2, 0.04));
    }
}
