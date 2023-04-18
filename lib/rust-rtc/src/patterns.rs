// Chapter 10 - Patterns

use crate::colors::{Color, WHITE};
use crate::matrices::Matrix4;
use crate::shapes::Shape;
use crate::tuples::Point;

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

    pub fn color_at_shape(&self, shape: &Shape, world_point: &Point) -> Color {
        // Convert world-space point to object-space point:
        let object_point = shape.transform.inverse() * world_point;
        return self.pattern_at(&object_point);
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
        }
    }
}

pub fn pattern_at(pattern: &Pattern, object_point: &Point) -> Color {
    pattern.pattern_at(object_point)
}

pub fn pattern_at_shape(pattern: &Pattern, shape: &Shape, world_point: &Point) -> Color {
    pattern.color_at_shape(shape, world_point)
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternEnum {
    SolidPattern(SolidPattern),
    StripePattern(StripePattern),
}

impl Default for PatternEnum {
    fn default() -> Self {
        PatternEnum::SolidPattern(SolidPattern::new(&WHITE))
    }
}

// *******************************
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SolidPattern {
    color: Color,
}

impl SolidPattern {
    pub fn new(color: &Color) -> SolidPattern {
        SolidPattern { color: *color }
    }
}

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

// *******************************
#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    a: Box<Pattern>,
    b: Box<Pattern>,
}

impl StripePattern {
    pub fn new(a: &Pattern, b: &Pattern) -> StripePattern {
        StripePattern { a: Box::new(a.clone()), b: Box::new(b.clone()) }
    }
}

impl PatternTrait for StripePattern {
    fn pattern_at(&self, local_point: &Point) -> Color {
        if local_point.x().floor() as i32 % 2 == 0 {
            let pattern_point = self.a.transform.inverse() * local_point;
            self.a.pattern_at(&pattern_point)
        } else {
            let pattern_point = self.b.transform.inverse() * local_point;
            self.b.pattern_at(&pattern_point)
        }
    }
}

impl Pattern {
    pub fn stripe_pattern(a: &Pattern, b: &Pattern) -> Pattern {
        Pattern {
            pattern: PatternEnum::StripePattern(
                StripePattern::new(a, b)),
            ..Default::default()
        }
    }
}

impl From<Color> for Pattern {
    fn from(value: Color) -> Self {
        Pattern::solid_pattern(&value)
    }
}

pub fn stripe_pattern(a: &Pattern, b: &Pattern) -> Pattern {
    Pattern::stripe_pattern(a, b)
}

#[cfg(test)]
mod tests {
    use crate::canvas::{canvas};
    use crate::colors::{WHITE, BLACK, GREEN, RED};
    use crate::shapes::sphere;
    use crate::transformations::{scaling, translation};
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
        let pattern = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));

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
        let pattern = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 2.0, 0.0)), WHITE);
    }

    // A stripe pattern is constant in z
    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 2.0)), WHITE);
    }

    // A stripe pattern alternates in x
    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));
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
        let pattern = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));
        let c = pattern_at_shape(&pattern, &shape, &point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }

    // Stripes with a pattern transformation
    #[test]
    fn stripes_with_a_pattern_transformation() {
        let shape = sphere(1);
        let mut pattern = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));
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
        let mut pattern = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));
        pattern.set_transform(&translation(0.5, 0.0, 0.0));
        let c = pattern_at_shape(&pattern, &shape, &point(2.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }
    /*

    // Towards Generic Patterns:

    class TestPattern : public Pattern {
    public:
        TestPattern() = default;

    //    std::unique_ptr<Pattern<double>> clone() const override {
    //        return std::make_unique<TestPattern>(*this);
    //    }

        Color pattern_at(Point const & local_point) const override {
            return {local_point.x(), local_point.y(), local_point.z()};
        }

    protected:
        // https://stackoverflow.com/a/43263477
        virtual std::unique_ptr<Pattern> clone_impl() const override {
            return std::make_unique<TestPattern>(*this);
        };
    };

    TestPattern test_pattern() {
        return {};
    }

    // The default pattern transformation
    #[test]
    fn default_pattern_transformation() {
        let pattern = test_pattern();
        assert_eq!(pattern.transform(), identity4x4());
    }

    // Assigning a transformation
    #[test]
    fn assigning_a_transformation() {
        let pattern = test_pattern();
        set_pattern_transform(&pattern, translation(1.0, 2.0, 3.0));
        assert_eq!(pattern.transform(), translation(1.0, 2.0, 3.0));
    }

    // A pattern with an object transformation
    #[test]
    fn pattern_with_an_object_transformation() {
        let shape = sphere(1);
        set_transform(shape, scaling(2.0, 2.0, 2.0));
        let pattern = test_pattern();
        let c = pattern_at_shape(&pattern, shape, &point(2.0, 3.0, 4.0));
        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    // A pattern with a pattern transformation
    #[test]
    fn pattern_with_a_pattern_transformation() {
        let shape = sphere(1);
        let pattern = test_pattern();
        set_pattern_transform(&pattern, scaling(2.0, 2.0, 2.0));
        let c = pattern_at_shape(&pattern, shape, &point(2.0, 3.0, 4.0));
        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    // A pattern with an object and a pattern transformation
    #[test]
    fn pattern_with_an_object_and_a_pattern_transformation() {
        let shape = sphere(1);
        set_transform(shape, scaling(2.0, 2.0, 2.0));
        let pattern = test_pattern();
        set_pattern_transform(&pattern, translation(0.5, 1.0, 1.5));
        let c = pattern_at_shape(&pattern, shape, &point(2.5, 3.0, 3.5));
        assert_eq!(c, color(0.75, 0.5, 0.25));
    }

    // Gradients

    // A gradient linearly interpolates between colors
    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = gradient_pattern(WHITE, BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.0)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern_at(&pattern, &point(0.5, 0.0, 0.0)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.0)), color(0.25, 0.25, 0.25));
    }

    // Rings

    // A ring should extend in both x and z
    #[test]
    fn ring_extends_in_both_x_and_z() {
        let pattern = ring_pattern(WHITE, BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.708, 0.0, 0.708)), BLACK);
    }

    // 3D Checkers

    // Checkers should repeat in x
    #[test]
    fn checkers_repeats_in_x() {
        let pattern = checkers_pattern(WHITE, BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(1.01, 0.0, 0.0)), BLACK);
    }

    // Checkers should repeat in y
    #[test]
    fn checkers_repeats_in_y() {
        let pattern = checkers_pattern(WHITE, BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 1.01, 0.0)), BLACK);
    }

    // Checkers should repeat in z
    #[test]
    fn checkers_repeats_in_z() {
        let pattern = checkers_pattern(WHITE, BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 1.01)), BLACK);
    }

    // Radial Gradient in x, y, z
    #[test]
    fn radial_gradient_linearly_interpolates_between_colors() {
        let pattern = radial_gradient_pattern(WHITE, BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.0)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern_at(&pattern, &point(0.5, 0.0, 0.0)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.0)), color(0.25, 0.25, 0.25));
        EXPECT_TRUE(almost_equal(pattern_at(&pattern, &point(1.0 - EPSILON, 0.0, 0.0)), BLACK));

        let side = [](double radius){ return sqrt(radius * radius / 2.0); };

        // radially in x, z
        let x0 {side(0.25)};
        let x1 {side(0.5)};
        let x2 {side(0.75)};
        let x3 {side(1.0 - EPSILON)};
        EXPECT_TRUE(AlmostEqual(pattern_at(&pattern, &point(x0, 0.0, x0)), color(0.75, 0.75, 0.75)));
        EXPECT_TRUE(AlmostEqual(pattern_at(&pattern, &point(x1, 0.0, x1)), color(0.5, 0.5, 0.5)));
        EXPECT_TRUE(AlmostEqual(pattern_at(&pattern, &point(x2, 0.0, x2)), color(0.25, 0.25, 0.25)));
        EXPECT_TRUE(AlmostEqual(pattern_at(&pattern, &point(x3, 0.0, x3)), BLACK));
    }
*/
    // Nested Patterns
    #[test]
    fn nested_patterns() {
        let mut p0 = stripe_pattern(&solid_pattern(&WHITE), &solid_pattern(&BLACK));
        p0.set_transform(&scaling(0.5, 0.5, 0.5));    // four stripes per unit
        let mut p1 = stripe_pattern(&solid_pattern(&GREEN), &solid_pattern(&RED));
        p1.set_transform(&scaling(0.5, 0.5, 0.5));    // four stripes per unit
        let mut pattern = stripe_pattern(&p0, &p1);
        pattern.set_transform(&scaling(0.5, 0.5, 0.5));  // two stripes per unit

        assert_eq!(pattern_at(&pattern, &point(0.125, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.375, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.625, 0.0, 0.0)), GREEN);
        assert_eq!(pattern_at(&pattern, &point(0.875, 0.0, 0.0)), RED);
    }
/*
    // Blended Patterns
    #[test]
    fn blended_patterns() {
        let p0 = stripe_pattern(WHITE, BLACK);
        p0.set_transform(scaling(0.5, 0.5, 0.5));
        let p1 = stripe_pattern(WHITE, BLACK);
        p1.set_transform(scaling(0.5, 0.5, 0.5).then(rotation_y(std::numbers::pi / 2.0)));
        let pattern = blended_pattern(p0, p1);

        dump_pattern(pattern);

        let const grey {color(0.5, 0.5, 0.5)};
        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.25)), grey);
        assert_eq!(pattern_at(&pattern, &point(0.25, 0.0, 0.75)), WHITE);
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.25)), BLACK);
        assert_eq!(pattern_at(&pattern, &point(0.75, 0.0, 0.75)), grey);
    }

    // Perturbed Patterns
    #[test]
    fn perturbed_patterns() {
        let p0 = stripe_pattern(WHITE, BLACK);
        let pattern = perturbed_pattern(p0,
                                         1.9,  // scale
                                         4,    // num_octaves,
                                         0.9); // persistence
        dump_pattern(&pattern, 1024, 4.0);
    }

     */
}
