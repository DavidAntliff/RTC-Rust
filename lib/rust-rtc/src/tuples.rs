use glam::f64::DVec4;

use derive_more::{Add, Neg, Div};

#[derive(Debug, Default, PartialEq, Add, Neg, Div)]
pub struct Tuple(pub(crate) glam::f64::DVec4);

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(DVec4 {x, y, z, w})
    }

    // pub fn from_inner(inner: DVec4) -> Self {
    //     Self(inner)
    // }

    pub fn x(&self) -> f64 { self.0.x }
    pub fn y(&self) -> f64 { self.0.y }
    pub fn z(&self) -> f64 { self.0.z }
    pub fn w(&self) -> f64 { self.0.w }
    pub fn at(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.0.x),
            1 => Some(self.0.y),
            2 => Some(self.0.z),
            3 => Some(self.0.w),
            _ => None,
        }
    }

    pub fn is_point(&self) -> bool {
        self.0.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.0.w == 0.0
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length
    pub fn magnitude(&self) -> f64 {
        self.0.length()
    }

    pub fn normalize(&self) -> Self {
        Self(self.0.normalize())
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0.dot(rhs.0)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        // 3D cross-product, ignore .w
        Self(glam::f64::DVec4 {
            x: self.0.y * rhs.0.z - rhs.0.y * self.0.z,
            y: self.0.z * rhs.0.x - rhs.0.z * self.0.x,
            z: self.0.x * rhs.0.y - rhs.0.x * self.0.y,
            w: 0.0
        })
    }
}

macro_rules! tuple_muls {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Tuple;
            fn mul(self, rhs: $rhs) -> Tuple {
                Tuple(self.0 * rhs.0)
            }
        }
    }
}

tuple_muls!(Tuple, Tuple);
tuple_muls!(Tuple, &Tuple);
tuple_muls!(&Tuple, Tuple);
tuple_muls!(&Tuple, &Tuple);

macro_rules! tuple_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Tuple;
            fn mul(self, rhs: $rhs) -> Tuple {
                Tuple(self.0 * rhs)
            }
        }
    }
}

tuple_mul!(Tuple, f64);
//tuple_mul!(Tuple, &f64);
tuple_mul!(&Tuple, f64);
//tuple_mul!(&Tuple, &f64);

impl std::ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        rhs * self
    }
}

macro_rules! tuple_sub {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = Tuple;
            fn sub(self, rhs: $rhs) -> Tuple {
                Tuple(self.0 - rhs.0)
            }
        }
    }
}

tuple_sub!(Tuple, Tuple);
tuple_sub!(Tuple, &Tuple);
tuple_sub!(&Tuple, Tuple);
tuple_sub!(&Tuple, &Tuple);

impl From<DVec4> for Tuple {
    fn from(value: DVec4) -> Self {
        Tuple(value)
    }
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple::new(x, y, z, w)
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn magnitude(v: &Tuple) -> f64 {
    v.magnitude()
}

pub fn normalize(v: &Tuple) -> Tuple {
    v.normalize()
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a.dot(b)
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    a.cross(b)
}

pub fn reflect(incoming: &Tuple, normal: &Tuple) -> Tuple {
    incoming - normal * 2.0 * dot(incoming, normal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, AbsDiffEq, RelativeEq};

    impl AbsDiffEq for Tuple {
        type Epsilon = f64;

        fn default_epsilon() -> f64 {
            f64::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
            self.0.abs_diff_eq(other.0, epsilon)
        }
    }

    impl RelativeEq for Tuple {
        fn default_max_relative() -> f64 {
            f64::default_max_relative()
        }

        fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
            f64::relative_eq(&self.0.x, &other.0.x, epsilon, max_relative) &&
                f64::relative_eq(&self.0.y, &other.0.y, epsilon, max_relative) &&
                f64::relative_eq(&self.0.z, &other.0.z, epsilon, max_relative) &&
                f64::relative_eq(&self.0.w, &other.0.w, epsilon, max_relative)
        }
    }

    // Conversion from a DVec4
    #[test]
    fn from_dvec4() {
        let a = Tuple::from(DVec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(a.x(), 1.0);
        assert_eq!(a.y(), 2.0);
        assert_eq!(a.z(), 3.0);
        assert_eq!(a.w(), 4.0);
    }

    // The default Tuple is all zero
    #[test]
    fn default_tuple() {
        let a = Tuple::default();
        assert_eq!(a.x(), 0.0);
        assert_eq!(a.y(), 0.0);
        assert_eq!(a.z(), 0.0);
        assert_eq!(a.w(), 0.0);
    }

    // Access via .at() function.
    // Invalid access via .at() returns empty optional
    #[test]
    fn at_access() {
        let a = tuple(1.0, 2.0, 3.0, 4.0);
        //assert!(a.at(-1).is_none());
        assert_eq!(a.at(0), Some(1.0));
        assert_eq!(a.at(1), Some(2.0));
        assert_eq!(a.at(2), Some(3.0));
        assert_eq!(a.at(3), Some(4.0));
        assert!(a.at(4).is_none());
        assert!(a.at(99).is_none());
    }

    // A tuple with w=1.0 is a point
    #[test]
    fn tuple_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 1.0);

        assert_eq!(a.at(0), Some(4.3));
        assert_eq!(a.at(1), Some(-4.2));
        assert_eq!(a.at(2), Some(3.1));
        assert_eq!(a.at(3), Some(1.0));

        assert_eq!(a.at(0).unwrap(), 4.3);
        assert_eq!(a.at(1).unwrap(), -4.2);
        assert_eq!(a.at(2).unwrap(), 3.1);
        assert_eq!(a.at(3).unwrap(), 1.0);

        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    // A tuple with w=0 is a vector
    #[test]
    fn tuple_is_a_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    // point() creates tuples with w=1
    #[test]
    fn point_creates_tuple() {
        let p = point(4.0, -4.0, 3.0);
        assert!(p.is_point());
        assert_eq!(p, tuple(4.0, -4.0, 3.0, 1.0));
    }

    // vector() creates tuples with w=0
    #[test]
    fn vector_creates_tuple() {
        let v = vector(4.0, -4.0, 3.0);
        assert!(v.is_vector());
        assert_eq!(v, tuple(4.0, -4.0, 3.0, 0.0));
    }

    // Adding two tuples
    #[test]
    fn adding_two_tuples() {
        let a1 = tuple(3., -2., 5., 1.);
        let a2 = tuple(-2., 3., 1., 0.);
        assert_eq!(a1 + a2, tuple(1., 1., 6., 1.));
    }

    // Subtracting two points
    #[test]
    fn subtracting_two_points() {
        let p1 = point(3., 2., 1.);
        let p2 = point(5., 6., 7.);
        assert_eq!(p1 - p2, vector(-2., -4., -6.));
    }

    // Subtracting a vector from a point
    #[test]
    fn subtracting_vector_from_point() {
        let p = point(3., 2., 1.);
        let v = vector(5., 6., 7.);
        assert_eq!(p - v, point(-2., -4., -6.));
    }

    // Subtracting two vectors
    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3., 2., 1.);
        let v2 = vector(5., 6., 7.);
        assert_eq!(v1 - v2, vector(-2., -4., -6.));
    }

    // Subtracting a vector from the zero vector
    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = vector(0., 0., 0.);
        let v = vector(1., -2., 3.);
        assert_eq!(zero - v, vector(-1., 2., -3.));
    }

    // Negating a tuple
    #[test]
    fn negating_a_tuple() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(-a, tuple(-1., 2., -3., 4.));
    }

    // Multiplying a tuple by a scalar
    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(a * 3.5, tuple(3.5, -7., 10.5, -14.));
    }

    #[test]
    fn multiplying_tuple_by_scalar_prefix() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(3.5 * a, tuple(3.5, -7., 10.5, -14.));
    }

    // Multiplying a tuple by a fraction
    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(a * 0.5, tuple(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn multiplying_tuple_by_fraction_prefix() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(0.5 * a, tuple(0.5, -1., 1.5, -2.));
    }

    // Dividing a tuple by a scalar
    #[test]
    fn dividing_tuple_by_scalar() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(a / 2.0, tuple(0.5, -1., 1.5, -2.));
    }

    // Computing the magnitude of vector(1, 0, 0)
    #[test]
    fn compute_magnitude_vector_1_0_0() {
        let v = vector(1., 0., 0.);
        assert_eq!(magnitude(&v), 1.);
    }

    // Computing the magnitude of vector(0, 1, 0)
    #[test]
    fn compute_magnitude_vector_0_1_0() {
        let v = vector(0., 1., 0.);
        assert_eq!(magnitude(&v), 1.);
    }

    // Computing the magnitude of vector(0, 0, 1)
    #[test]
    fn compute_magnitude_vector_0_0_1() {
        let v = vector(0., 0., 1.);
        assert_eq!(magnitude(&v), 1.);
    }

    // Computing the magnitude of vector(1, 2, 3)
    #[test]
    fn compute_magnitude_vector_1_2_3() {
        let v = vector(1., 2., 3.);
        assert_eq!(magnitude(&v), f64::sqrt(14.0));
        assert_eq!(magnitude(&v), (14.0 as f64).sqrt());  // equivalent
    }

    // Computing the magnitude of vector(-1, -2, -3)
    #[test]
    fn compute_magnitude_vector_n1_n2_n3() {
        let v = vector(-1., -2., -3.);
        assert_eq!(magnitude(&v), f64::sqrt(14.0));
    }

    // Normalizing vector(4, 0, 0) gives (1, 0, 0)
    #[test]
    fn normalizing_vector_4_0_0() {
        let v = vector(4., 0., 0.);
        assert_eq!(normalize(&v), vector(1., 0., 0.));
    }

    // Normalizing vector(1, 2, 3)
    #[test]
    fn normalizing_vector_1_2_3() {
        let v = vector(1., 2., 3.);
        let sqrt14 = f64::sqrt(14.);
        assert_eq!(normalize(&v), vector(1. / sqrt14, 2. / sqrt14, 3. / sqrt14));
    }

    // The magnitude of a normalized vector
    #[test]
    fn magnitude_of_normalized_vector() {
        let v = vector(1., 2., 3.);
        let norm = normalize(&v);
        assert_eq!(magnitude(&norm), 1.0);
    }

    // The dot product of two tuples
    #[test]
    fn dot_product_of_two_tuples() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);
        assert_eq!(dot(&a, &b), 20.0);
    }

    // The cross product of two vectors
    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);
        assert_eq!(cross(&a, &b), vector(-1., 2., -1.));
        assert_eq!(cross(&b, &a), vector(1., -2., 1.));
    }

    // Chapter 6 - Light and Shading

    // Reflecting a vector approaching at 45 degrees
    #[test]
    fn reflecting_vector_at_45_degrees() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }

    // Reflecting a vector off a slanted surface
    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let k = f64::sqrt(2.0) / 2.0;
        let n = vector(k, k, 0.0);
        let r = reflect(&v, &n);
        assert_relative_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
