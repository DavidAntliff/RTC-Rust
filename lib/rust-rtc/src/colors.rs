use super::tuples::{Tuple};
use derive_more::{Add, Sub, Neg, Mul};

#[derive(Debug, Default, PartialEq, Copy, Clone, Add, Sub, Neg, Mul)]
pub struct Color(Tuple);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Tuple::new(r, g, b, 0.0))
    }

    pub fn red(&self) -> f64 { self.0.x() }
    pub fn green(&self) -> f64 { self.0.y() }
    pub fn blue(&self) -> f64 { self.0.z() }
}

macro_rules! color_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Color;
            fn mul(self, rhs: $rhs) -> Color {
                Color(&self.0 * &rhs.0)
            }
        }
    }
}

color_mul!(Color, Color);
color_mul!(Color, &Color);
color_mul!(&Color, Color);
color_mul!(&Color, &Color);

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}

/// Hadamard or Shur Product
pub fn hadamard(lhs: &Color, rhs: &Color) -> Color {
    lhs * rhs
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, AbsDiffEq, RelativeEq};

    impl AbsDiffEq for Color {
        type Epsilon = f64;

        fn default_epsilon() -> f64 {
            f64::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
            self.0.0.abs_diff_eq(other.0.0, epsilon)
        }
    }

    impl RelativeEq for Color {
        fn default_max_relative() -> f64 {
            f64::default_max_relative()
        }

        fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
            f64::relative_eq(&self.0.0.x, &other.0.0.x, epsilon, max_relative) &&
                f64::relative_eq(&self.0.0.y, &other.0.0.y, epsilon, max_relative) &&
                f64::relative_eq(&self.0.0.z, &other.0.0.z, epsilon, max_relative) &&
                f64::relative_eq(&self.0.0.w, &other.0.0.w, epsilon, max_relative)
        }
    }

    // Colors are (red, green, blue) tuples
    #[test]
    fn colors_are_tuples() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    // Adding colors
    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    // Subtracting colors
    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_relative_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }

    // Multiplying a color by a scalar
    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    // Multiplying colors
    #[test]
    fn multiplying_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_relative_eq!(c1 * c2, color(0.9, 0.2, 0.04));
        assert_relative_eq!(hadamard(&c1, &c2), color(0.9, 0.2, 0.04));
    }
}
