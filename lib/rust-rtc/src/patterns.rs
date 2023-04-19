// Chapter 10 - Patterns

use crate::colors::{Color, linear_blend, WHITE};
use crate::matrices::Matrix4;
use crate::perlin_noise;
use crate::shapes::Shape;
use crate::tuples::{Point, point};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Pattern {
    pattern: PatternEnum,
    transform: Matrix4,
}

impl Pattern {
    pub fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = *transform;
    }

    pub fn pattern_at(&self, object_point: &Point) -> Color {
        // Convert object-space point to pattern-space point:
        let pattern_point = self.transform.inverse() * object_point;
        self.pattern.pattern_at(&pattern_point)
    }

    pub fn pattern_at_shape(&self, shape: &Shape, world_point: &Point) -> Color {
        // Convert world-space point to object-space point:
        let object_point = shape.transform.inverse() * world_point;
        self.pattern_at(&object_point)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternEnum {
    SolidPattern(SolidPattern),
    StripePattern(StripePattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
    CheckersPattern(CheckersPattern),
    RadialGradientPattern(RadialGradientPattern),
    BlendedPattern(BlendedPattern),
    PerturbedPattern(PerturbedPattern),
}

impl Default for PatternEnum {
    fn default() -> Self {
        PatternEnum::SolidPattern(SolidPattern::new(&WHITE))
    }
}

trait PatternTrait {
    fn pattern_at(&self, local_point: &Point) -> Color;
}

impl PatternTrait for PatternEnum {
    fn pattern_at(&self, local_point: &Point) -> Color {
        match self {
            PatternEnum::SolidPattern(pattern) => pattern.pattern_at(local_point),
            PatternEnum::StripePattern(pattern) => pattern.pattern_at(local_point),
            PatternEnum::GradientPattern(pattern) => pattern.pattern_at(local_point),
            PatternEnum::RingPattern(pattern) => pattern.pattern_at(local_point),
            PatternEnum::CheckersPattern(pattern) => pattern.pattern_at(local_point),
            PatternEnum::RadialGradientPattern(pattern) => pattern.pattern_at(local_point),
            PatternEnum::BlendedPattern(pattern) => pattern.pattern_at(local_point),
            PatternEnum::PerturbedPattern(pattern) => pattern.pattern_at(local_point),
        }
    }
}

pub fn pattern_at(pattern: &Pattern, object_point: &Point) -> Color {
    pattern.pattern_at(object_point)
}

pub fn pattern_at_shape(pattern: &Pattern, shape: &Shape, world_point: &Point) -> Color {
    pattern.pattern_at_shape(shape, world_point)
}

// Trait to allow Patterns or Colors to be used
// as parameters to pattern factory functions
pub trait IntoPattern {
    fn into_pattern(&self) -> Pattern;
}

impl IntoPattern for Pattern {
    fn into_pattern(&self) -> Pattern {
        self.clone()
    }
}

impl IntoPattern for Color {
    fn into_pattern(&self) -> Pattern {
        Pattern::solid_pattern(self)
    }
}


// ------[ SolidPattern ]------
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SolidPattern {
    color: Color,
}

impl SolidPattern {
    pub fn new(color: &Color) -> SolidPattern {
        SolidPattern { color: *color }
    }
}

// TODO: generic factory functions that can accept
//       &Pattern or &Color parameters?

impl PatternTrait for SolidPattern {
    fn pattern_at(&self, _local_point: &Point) -> Color {
        self.color
    }
}

impl Pattern {
    pub fn solid_pattern(color: &Color) -> Pattern {
        Pattern {
            pattern: PatternEnum::SolidPattern(SolidPattern::new(color)),
            ..Default::default()
        }
    }
}

pub fn solid_pattern(color: &Color) -> Pattern {
    Pattern::solid_pattern(color)
}

impl From<Color> for Pattern {
    fn from(value: Color) -> Self {
        Pattern::solid_pattern(&value)
    }
}


// ------[ StripePattern ]------
#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl StripePattern {
    pub fn new<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> StripePattern {
        StripePattern { a: Box::new(a.into_pattern()), b: Box::new(b.into_pattern()) }
    }
}

impl PatternTrait for StripePattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        if local_point.x().floor() as i32 % 2 == 0 {
            let pattern_point = self.a.transform.inverse() * local_point;
            self.a.pattern.pattern_at(&pattern_point)
        } else {
            let pattern_point = self.b.transform.inverse() * local_point;
            self.b.pattern.pattern_at(&pattern_point)
        }
    }
}

impl Pattern {
       pub fn stripe_pattern<T, U>(a: &T, b: &U) -> Pattern
           where T: IntoPattern, U: IntoPattern {
        Pattern {
            pattern: PatternEnum::StripePattern(
                StripePattern::new(&a.into_pattern(), &b.into_pattern())),
            ..Default::default()
        }
    }
}

pub fn stripe_pattern<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> Pattern {
    Pattern::stripe_pattern(&a.into_pattern(), &b.into_pattern())
}


// ------[ GradientPattern ]------
#[derive(Debug, PartialEq, Clone)]
pub struct GradientPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl GradientPattern {
        pub fn new<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> GradientPattern {
            GradientPattern { a: Box::new(a.into_pattern()), b: Box::new(b.into_pattern()) }
    }
}

impl PatternTrait for GradientPattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        let pattern_point_a = self.a.transform.inverse() * local_point;
        let pattern_point_b = self.b.transform.inverse() * local_point;
        linear_blend(local_point.x(),
                     &self.a.pattern.pattern_at(&pattern_point_a),
                     &self.b.pattern.pattern_at(&pattern_point_b))
    }
}

impl Pattern {
    pub fn gradient_pattern<T, U>(a: &T, b: &U) -> Pattern
        where T: IntoPattern, U: IntoPattern {
        Pattern {
            pattern: PatternEnum::GradientPattern(
                GradientPattern::new(&a.into_pattern(), &b.into_pattern())),
            ..Default::default()
        }
    }
}

pub fn gradient_pattern<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> Pattern {
    Pattern::gradient_pattern(&a.into_pattern(), &b.into_pattern())
}


// ------[ RingPattern ]------
#[derive(Debug, PartialEq, Clone)]
pub struct RingPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl RingPattern {
    pub fn new<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> RingPattern {
        RingPattern { a: Box::new(a.into_pattern()), b: Box::new(b.into_pattern()) }
    }
}

impl PatternTrait for RingPattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        let distance  = f64::sqrt(local_point.x() * local_point.x()
                                + local_point.z() * local_point.z());
        if distance.floor() as i32  % 2 == 0 {
            let pattern_point_a = self.a.transform.inverse() * local_point;
            self.a.pattern.pattern_at(&pattern_point_a)
        } else {
            let pattern_point_b = self.b.transform.inverse() * local_point;
            self.b.pattern.pattern_at(&pattern_point_b)
        }
    }
}

impl Pattern {
    pub fn ring_pattern<T, U>(a: &T, b: &U) -> Pattern
        where T: IntoPattern, U: IntoPattern {
        Pattern {
            pattern: PatternEnum::RingPattern(
                RingPattern::new(a, b)),
            ..Default::default()
        }
    }
}

pub fn ring_pattern<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> Pattern {
    Pattern::ring_pattern(&a.into_pattern(), &b.into_pattern())
}


// ------[ CheckersPattern ]------
#[derive(Debug, PartialEq, Clone)]
pub struct CheckersPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl CheckersPattern {
    pub fn new<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> CheckersPattern {
        CheckersPattern { a: Box::new(a.into_pattern()), b: Box::new(b.into_pattern()) }
    }
}

impl PatternTrait for CheckersPattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        let sum = local_point.x().floor() +
                       local_point.y().floor() +
                       local_point.z().floor();
        if sum.floor() as i32 % 2 == 0 {
            let pattern_point = self.a.transform.inverse() * local_point;
            self.a.pattern.pattern_at(&pattern_point)
        } else {
            let pattern_point = self.b.transform.inverse() * local_point;
            self.b.pattern.pattern_at(&pattern_point)
        }
    }
}

impl Pattern {
    pub fn checkers_pattern<T, U>(a: &T, b: &U) -> Pattern
        where T: IntoPattern, U: IntoPattern {
        Pattern {
            pattern: PatternEnum::CheckersPattern(
                CheckersPattern::new(a, b)),
            ..Default::default()
        }
    }
}

pub fn checkers_pattern<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> Pattern {
    Pattern::checkers_pattern(&a.into_pattern(), &b.into_pattern())
}


// ------[ RadialGradientPattern ]------
#[derive(Debug, PartialEq, Clone)]
pub struct RadialGradientPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
    y_factor: f64,
}

impl RadialGradientPattern {
    pub fn new<T: IntoPattern, U: IntoPattern>(a: &T, b: &U, y_factor: f64) -> RadialGradientPattern {
        RadialGradientPattern {
            a: Box::new(a.into_pattern()),
            b: Box::new(b.into_pattern()),
            y_factor}
    }
}

impl PatternTrait for RadialGradientPattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        let distance = f64::sqrt(local_point.x() * local_point.x()
            + self.y_factor * local_point.y() * local_point.y() +
            local_point.z() * local_point.z());
        let pattern_point_a = self.a.transform.inverse() * local_point;
        let pattern_point_b = self.b.transform.inverse() * local_point;
        linear_blend(distance,
                     &self.a.pattern.pattern_at(&pattern_point_a),
                     &self.b.pattern.pattern_at(&pattern_point_b))
    }
}

impl Pattern {
    pub fn radial_gradient_pattern<T, U>(a: &T, b: &U, y_factor: f64) -> Pattern
        where T: IntoPattern, U: IntoPattern {
        Pattern {
            pattern: PatternEnum::RadialGradientPattern(
                RadialGradientPattern::new(a, b, y_factor)),
            ..Default::default()
        }
    }
}

// TODO: consider a newtype YFactor(f64) that has a default, allowing:
//   //
//   radial_gradient_pattern(&WHITE, &BLACK, YFactor::default)
//
pub fn radial_gradient_pattern<T: IntoPattern, U: IntoPattern>(a: &T, b: &U, y_factor: f64) -> Pattern {
    Pattern::radial_gradient_pattern(&a.into_pattern(), &b.into_pattern(), y_factor)
}


// ------[ BlendedPattern ]------
#[derive(Debug, PartialEq, Clone)]
pub struct BlendedPattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl BlendedPattern {
    pub fn new<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> BlendedPattern {
        BlendedPattern {
            a: Box::new(a.into_pattern()),
            b: Box::new(b.into_pattern()),
        }
    }
}

impl PatternTrait for BlendedPattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        let pattern_point_a = self.a.transform.inverse() * local_point;
        let pattern_point_b = self.b.transform.inverse() * local_point;
        let color_a = self.a.pattern.pattern_at(&pattern_point_a);
        let color_b = self.b.pattern.pattern_at(&pattern_point_b);
        (color_a + color_b) / 2.0
    }
}

impl Pattern {
    pub fn blended_pattern<T, U>(a: &T, b: &U) -> Pattern
        where T: IntoPattern, U: IntoPattern {
        Pattern {
            pattern: PatternEnum::BlendedPattern(
                BlendedPattern::new(a, b)),
            ..Default::default()
        }
    }
}

pub fn blended_pattern<T: IntoPattern, U: IntoPattern>(a: &T, b: &U) -> Pattern {
    Pattern::blended_pattern(&a.into_pattern(), &b.into_pattern())
}


// ------[ PerturbedPattern ]------
#[derive(Debug, PartialEq, Clone)]
pub struct PerturbedPattern {
    a: Box<Pattern>,
    scale: f64,
    num_octaves: u32,
    persistence: f64,
}

impl Default for PerturbedPattern {
    fn default() -> Self {
        PerturbedPattern {
            a: Default::default(),
            scale: 0.5,
            num_octaves: 1,
            persistence: 0.9,
        }
    }
}

impl PerturbedPattern {
    pub fn new<T: IntoPattern>(a: &T, scale: f64, num_octaves: u32, persistence: f64) -> PerturbedPattern {
        PerturbedPattern {
            a: Box::new(a.into_pattern()),
            scale,
            num_octaves,
            persistence,
        }
    }
}

impl PatternTrait for PerturbedPattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        let new_x = local_point.x() + perlin_noise::octave_perlin(local_point.x(), local_point.y(), local_point.z(), self.num_octaves, self.persistence) * self.scale;
        let new_y = local_point.y() + perlin_noise::octave_perlin(local_point.x(), local_point.y(), local_point.z() + 1.0, self.num_octaves, self.persistence) * self.scale;
        let new_z = local_point.z() + perlin_noise::octave_perlin(local_point.x(), local_point.y(), local_point.z() + 2.0, self.num_octaves, self.persistence) * self.scale;
        let perturbed_point = point(new_x, new_y, new_z);

        let pattern_point = self.a.transform.inverse() * perturbed_point;
        self.a.pattern.pattern_at(&pattern_point)
    }
}

impl Pattern {
    pub fn perturbed_pattern<T>(a: &T, scale: f64, num_octaves: u32, persistence: f64) -> Pattern
        where T: IntoPattern {
        Pattern {
            pattern: PatternEnum::PerturbedPattern(
                PerturbedPattern::new(a, scale, num_octaves, persistence)),
            ..Default::default()
        }
    }
}

pub fn perturbed_pattern<T: IntoPattern>(a: &T, scale: f64, num_octaves: u32, persistence: f64) -> Pattern {
    Pattern::perturbed_pattern(&a.into_pattern(), scale, num_octaves, persistence)
}


#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use approx::assert_relative_eq;
    use crate::canvas::{canvas};
    use crate::colors::{WHITE, BLACK, GREEN, RED, color, GREY50};
    use crate::math::EPSILON;
    use crate::shapes::sphere;
    use crate::transformations::{rotation_y, scaling, translation};
    use crate::tuples::point;
    use super::*;

    fn dump_pattern(pattern: &Pattern, filename: &str, size: u32, scale: f64) {
        let mut image = canvas(size, size);
        for y in 0..image.height {
            for x in 0..image.width {
                let dx = scale * x as f64 / image.width as f64;
                let dy  = scale * y as f64 / image.height as f64;
                let v = pattern_at(&pattern, &point(dx, 0.0, dy));
                image.write_pixel(x, y, &v);
            }
        }
        image.to_ppm_file(filename);
    }

    // Creating a solid pattern
    #[test]
    fn creating_a_solid_pattern() {
        let pattern = solid_pattern(&WHITE);
        assert!(matches!(pattern.pattern, PatternEnum::SolidPattern { .. }));
        let inner = match pattern.pattern {
            PatternEnum::SolidPattern(inner) => inner,
            _ => unreachable!(),
        };
        assert_eq!(inner.color, WHITE);

        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(&point(1.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 1.0)), WHITE);

        dump_pattern(&pattern, "creating_a_solid_pattern.ppm", 100, 1.0);
    }

    // Creating a stripe pattern
    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = stripe_pattern(&WHITE, &BLACK);

        // First level is another pattern
        assert!(matches!(pattern.pattern, PatternEnum::StripePattern { .. }));
        let inner = match &pattern.pattern {
            PatternEnum::StripePattern(inner) => inner,
            _ => unreachable!(),
        };

        // Second level is a solid pattern
        assert!(matches!((*(inner.a)).pattern, PatternEnum::SolidPattern { .. }));
        assert!(matches!((*(inner.b)).pattern, PatternEnum::SolidPattern { .. }));
        let inner_a = match (*(inner.a)).pattern {
            PatternEnum::SolidPattern(inner) => inner,
            _ => unreachable!(),
        };
        assert_eq!(inner_a.color, WHITE);
        let inner_b = match (*(inner.b)).pattern {
            PatternEnum::SolidPattern(inner) => inner,
            _ => unreachable!(),
        };
        assert_eq!(inner_b.color, BLACK);

        dump_pattern(&pattern, "creating_a_stripe_pattern.ppm", 100, 2.0);
    }

    // A stripe pattern is constant in y
    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = stripe_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 2.0, 0.0)), WHITE);
    }

    // A stripe pattern is constant in z
    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 2.0)), WHITE);
    }

    // A stripe pattern alternates in x
    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(-1.1, 0.0, 0.0)), WHITE);
    }

    // Stripes with an object transformation
    #[test]
    fn stripes_with_an_object_transformation() {
        let mut shape = sphere(1);
        shape.set_transform(&scaling(2.0, 2.0, 2.0));
        let pattern = stripe_pattern(&WHITE, &BLACK);
        let c = pattern_at_shape(&pattern, &shape, &point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }

    // Stripes with a pattern transformation
    #[test]
    fn stripes_with_a_pattern_transformation() {
        let shape = sphere(1);
        let mut pattern = stripe_pattern(&WHITE, &BLACK);
        pattern.set_transform(&scaling(2.0, 2.0, 2.0));
        let c = pattern_at_shape(&pattern, &shape, &point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);

        dump_pattern(&pattern, "stripes_with_a_pattern_transformation.ppm", 100, 4.0);
    }

    // Stripes with an object and a pattern transformation
    #[test]
    fn stripes_with_an_object_and_a_pattern_transformation() {
        let mut shape = sphere(1);
        shape.set_transform(&scaling(2.0, 2.0, 2.0));
        let mut pattern = stripe_pattern(&WHITE, &BLACK);
        pattern.set_transform(&translation(0.5, 0.0, 0.0));
        let c = pattern_at_shape(&pattern, &shape, &point(2.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }

    // Interfaces for creating stripe patterns
    #[test]
    fn create_stripe_pattern_in_various_ways() {
        let _p = stripe_pattern(&WHITE, &BLACK);
        let _p = StripePattern::new(&WHITE, &BLACK);
        let _p = Pattern::stripe_pattern(&WHITE, &BLACK);

        let q = solid_pattern(&RED);
        let _p = stripe_pattern(&q, &q);
        let _p = StripePattern::new(&q, &q);
        let _p = Pattern::stripe_pattern(&q, &q);

        // mixed parameters
        let _p = stripe_pattern(&WHITE, &q);
        let _p = stripe_pattern(&q, &BLACK);
        let _p = StripePattern::new(&WHITE, &q);
        let _p = StripePattern::new(&q, &BLACK);
        let _p = Pattern::stripe_pattern(&WHITE, &q);
        let _p = Pattern::stripe_pattern(&q, &BLACK);
    }

    // Gradients

    // A gradient linearly interpolates between colors
    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = gradient_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.0)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern_at(&pattern, &point(0.5, 0.0, 0.0)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.0)), color(0.25, 0.25, 0.25));

        dump_pattern(&pattern, "gradient_linearly_interpolates_between_colors.ppm", 100, 1.0);
    }

    // Rings

    // A ring should extend in both x and z
    #[test]
    fn ring_extends_in_both_x_and_z() {
        let pattern = ring_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.708, 0.0, 0.708)), BLACK);

        dump_pattern(&pattern, "ring_extends_in_both_x_and_z.ppm", 100, 4.0);
    }

    // 3D Checkers

    // Checkers should repeat in x
    #[test]
    fn checkers_repeats_in_x() {
        let pattern = checkers_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(1.01, 0.0, 0.0)), BLACK);

        dump_pattern(&pattern, "checkers_repeats_in_x.ppm", 100, 4.0);
    }

    // Checkers should repeat in y
    #[test]
    fn checkers_repeats_in_y() {
        let pattern = checkers_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 1.01, 0.0)), BLACK);
    }

    // Checkers should repeat in z
    #[test]
    fn checkers_repeats_in_z() {
        let pattern = checkers_pattern(&WHITE, &BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 1.01)), BLACK);
    }

    // Radial Gradient in x, y, z
    #[test]
    fn radial_gradient_linearly_interpolates_between_colors() {
        let pattern = radial_gradient_pattern(&WHITE, &BLACK, 0.0);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.0)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern_at(&pattern, &point(0.5, 0.0, 0.0)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.0)), color(0.25, 0.25, 0.25));
        assert_relative_eq!(pattern_at(&pattern, &point(1.0 - EPSILON, 0.0, 0.0)), BLACK, epsilon=1e-5);

        fn side(radius: f64) -> f64 {
            f64::sqrt(radius * radius / 2.0)
        }

        // radially in x, z
        let x0 = side(0.25);
        let x1 = side(0.5);
        let x2 = side(0.75);
        let x3 = side(1.0 - EPSILON);
        assert_relative_eq!(pattern_at(&pattern, &point(x0, 0.0, x0)), color(0.75, 0.75, 0.75));
        assert_relative_eq!(pattern_at(&pattern, &point(x1, 0.0, x1)), color(0.5, 0.5, 0.5));
        assert_relative_eq!(pattern_at(&pattern, &point(x2, 0.0, x2)), color(0.25, 0.25, 0.25));
        assert_relative_eq!(pattern_at(&pattern, &point(x3, 0.0, x3)), BLACK, epsilon=1e-5);

        dump_pattern(&pattern, "radial_gradient_linearly_interpolates_between_colors.ppm", 100, 4.0);
    }

    // Nested Patterns
    #[test]
    fn nested_patterns() {
        let mut p0 = stripe_pattern(&WHITE, &BLACK);
        p0.set_transform(&scaling(0.5, 0.5, 0.5));    // four stripes per unit
        let mut p1 = stripe_pattern(&GREEN, &RED);
        p1.set_transform(&scaling(0.5, 0.5, 0.5));    // four stripes per unit
        let mut pattern = stripe_pattern(&p0, &p1);
        pattern.set_transform(&scaling(0.5, 0.5, 0.5));  // two stripes per unit

        assert_eq!(pattern_at(&pattern, &point(0.125, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.375, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.625, 0.0, 0.0)), GREEN);
        assert_eq!(pattern_at(&pattern, &point(0.875, 0.0, 0.0)), RED);
    }

    // Blended Patterns
    #[test]
    fn blended_patterns() {
        let mut p0 = stripe_pattern(&WHITE, &BLACK);
        p0.set_transform(&scaling(0.5, 0.5, 0.5));
        let mut p1 = stripe_pattern(&WHITE, &BLACK);
        p1.set_transform(&scaling(0.5, 0.5, 0.5).then(&rotation_y(PI / 2.0)));
        let pattern = blended_pattern(&p0, &p1);

        dump_pattern(&pattern, "blended_patterns.ppm", 100, 4.0);

        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.25)), GREY50);
        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.75)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.25)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.75)), GREY50);
    }

    // Perturbed Patterns
    #[test]
    fn perturbed_patterns() {
        let p0 = stripe_pattern(&WHITE, &BLACK);
        let pattern = perturbed_pattern(&p0,
                                              1.9,  // scale
                                        4,    // num_octaves,
                                         0.9); // persistence
        dump_pattern(&pattern, "perturbed_patterns.ppm", 100, 4.0);
    }
}
