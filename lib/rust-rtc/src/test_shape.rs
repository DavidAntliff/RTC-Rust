/* TODO:

This is problematic because we can't do the following
two things at the same time:

1. implement interior mutability, allowing the TestShape to act like a Mock
   and set internal state (saved_ray) in a non-mut-self function,
2. implement Copy, required by Shape / ShapeEnum, because we need Clone also.

It seems that `Cell` was deliberately made non-Copy to avoid
unexpected issues, but I can't find a way to create a Copy-able
similar mechanism to implement interior mutability.

Rust Book mentions the problem but doesn't consider the Copy
trait: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
*/

/*
use crate::intersections::Intersections;
use crate::rays::Ray;
use crate::shapes::{Shape, ShapeEnum};
use crate::tuples::{Point, vector, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TestShape {
    saved_ray: std::cell::Cell<Ray>,
    ///////_marker: std::marker::PhantomData<*const ()>,
}

// impl Copy for TestShape {}

// impl TestShape {
//     fn copy(&self) -> TestShape {
//         TestShape {
//             saved_ray: std::cell::Cell::new(self.cell_field.get()),
//         }
//     }
// }

impl TestShape {
    pub fn new() -> TestShape {
        TestShape { saved_ray: Ray { origin: Default::default(), direction: Default::default() } }
    }

    pub fn local_intersect(&self, local_ray: &Ray) -> Intersections {
        unsafe {
            let mut f = &self.saved_ray;
            f = *local_ray;
        }
        //self.saved_ray = *local_ray;
        vec![]
    }

    pub fn local_normal_at(&self, local_point: &Point) -> Vector {
        vector(local_point.x(), local_point.y(), local_point.z())
    }
}

impl Shape {
    fn test_shape() -> Shape {
        Shape {
            shape: ShapeEnum::TestShape(TestShape::new()),
            ..Default::default()
        }
    }
}

pub fn test_shape() -> Shape { Shape::test_shape() }
*/
/*
#[cfg(test)]
mod tests {
    use crate::matrices::identity4;
    use super::*;

// A shape's default transformation
    #[test]
    fn shape_has_default_transformation() {
        let s = test_shape();
        assert_eq!(s.transform, identity4());
    }

*/


    /*

// Changing a shape's transformation
TEST(TestShapes, changing_shape_transformation) {
    auto s = test_shape();
    auto t = translation(2.0, 3.0, 4.0);
    set_transform(s, t);
    EXPECT_EQ(s.transform(), t);
}

// A shape has a default material
TEST(TestShapes, shape_has_default_material) {
    auto s = test_shape();
    auto m = s.material();
    EXPECT_EQ(m, material());
}

// A shape may be assigned a material
TEST(TestShapes, shape_may_be_assigned_material) {
    auto s = test_shape();
    auto m = material();
    m.set_ambient(1.0);
    s.set_material(m);
    EXPECT_EQ(s.material(), m);
}

// Intersecting a scaled shape with a ray
TEST(TestShapes, intersecting_scaled_shape_with_ray) {
    auto r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    auto s = test_shape();
    set_transform(s, scaling(2.0, 2.0, 2.0));
    auto xs = intersect(s, r);
    EXPECT_EQ(s.saved_ray.origin(), point(0.0, 0.0, -2.5));
    EXPECT_EQ(s.saved_ray.direction(), vector(0.0, 0.0, 0.5));
}

// Intersecting a translated shape with a ray
TEST(TestShapes, intersecting_translated_shape_with_ray) {
    auto r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    auto s = test_shape();
    set_transform(s, translation(5.0, 0.0, 0.0));
    auto xs = intersect(s, r);
    EXPECT_EQ(s.saved_ray.origin(), point(-5.0, 0.0, -5.0));
    EXPECT_EQ(s.saved_ray.direction(), vector(0.0, 0.0, 1.0));
}

// Computing the normal on a translated shape
TEST(TestShapes, computing_normal_on_translated_shape) {
    auto s = test_shape();
    set_transform(s, translation(0.0, 1.0, 0.0));
    auto n = normal_at(s, point(0.0, 1.70711, -0.70711));
    EXPECT_TRUE(almost_equal(n, vector(0.0, 0.70711, -0.70711), 1e-4));
}

// Computing the normal on a transformed shape
TEST(TestShapes, computing_normal_on_transformed_shape) {
    auto s = test_shape();
    auto m = scaling(1.0, 0.5, 1.0) * rotation_z(std::numbers::pi / 5.0);
    set_transform(s, m);
    auto k = sqrt(2.0) / 2.0;
    auto n = normal_at(s, point(0.0, k, -k));
    EXPECT_TRUE(almost_equal(n, vector(0.0, 0.970143, -0.242546), 1e-4));
}

}
 */
