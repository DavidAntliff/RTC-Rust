// Chapter 3: Matrices

use super::tuples::{Tuple};
use glam::f64::{DMat2, DMat3, DMat4};

#[derive(Debug, PartialEq)]
pub struct Matrix2(DMat2);

#[derive(Debug, PartialEq)]
pub struct Matrix3(DMat3);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix4(DMat4);

impl Matrix2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows_array(m: &[[f64; 2]; 2]) -> Self {
        let col0 = [m[0][0], m[1][0]];
        let col1 = [m[0][1], m[1][1]];
        Self(DMat2::from_cols_array_2d(&[col0, col1]))
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.0.col(col)[row]
    }

    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }
}

impl Default for Matrix2 {
    fn default() -> Self {
        Self(DMat2::IDENTITY)
    }
}

pub fn matrix2(m: &[[f64; 2]; 2]) -> Matrix2 {
    Matrix2::from_rows_array(m)
}

impl Matrix3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows_array(m: &[[f64; 3]; 3]) -> Self {
        let col0 = [m[0][0], m[1][0], m[2][0]];
        let col1 = [m[0][1], m[1][1], m[2][1]];
        let col2 = [m[0][2], m[1][2], m[2][2]];
        Self(DMat3::from_cols_array_2d(&[col0, col1, col2]))
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.0.col(col)[row]
    }

    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self(DMat3::IDENTITY)
    }
}

pub fn matrix3(m: &[[f64; 3]; 3]) -> Matrix3 {
    Matrix3::from_rows_array(m)
}

impl Matrix4 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows_array(m: &[[f64; 4]; 4]) -> Self {
        let col0 = [m[0][0], m[1][0], m[2][0], m[3][0]];
        let col1 = [m[0][1], m[1][1], m[2][1], m[3][1]];
        let col2 = [m[0][2], m[1][2], m[2][2], m[3][2]];
        let col3 = [m[0][3], m[1][3], m[2][3], m[3][3]];
        Self(DMat4::from_cols_array_2d(&[col0, col1, col2, col3]))
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.0.col(col)[row]
    }

    pub fn transpose(&self) -> Self {
        Self(self.0.transpose())
    }

    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

    // Fluent API support:
    pub fn then(&mut self, m: &Matrix4) -> Matrix4 {
        *self = m * *self;
        *self
    }
}

pub fn transpose(m: &Matrix4) -> Matrix4 {
    m.transpose()
}

pub fn is_invertible(m: &Matrix4) -> bool {
    m.is_invertible()
}

pub fn inverse(m: &Matrix4) -> Matrix4 {
    m.inverse()
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self(DMat4::IDENTITY)
    }
}

macro_rules! matrix4_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Matrix4;
            fn mul(self, rhs: $rhs) -> Matrix4 {
                Matrix4(self.0 * rhs.0)
            }
        }
    }
}

matrix4_mul!(Matrix4, Matrix4);
matrix4_mul!(Matrix4, &Matrix4);
matrix4_mul!(&Matrix4, Matrix4);
matrix4_mul!(&Matrix4, &Matrix4);

macro_rules! matrix4_tuple_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Tuple;
            fn mul(self, rhs: $rhs) -> Tuple {
                Tuple(self.0 * rhs.0)
            }
        }
    }
}

matrix4_tuple_mul!(Matrix4, Tuple);
matrix4_tuple_mul!(Matrix4, &Tuple);
matrix4_tuple_mul!(&Matrix4, Tuple);
matrix4_tuple_mul!(&Matrix4, &Tuple);

pub fn matrix4(m: &[[f64; 4]; 4]) -> Matrix4 {
    Matrix4::from_rows_array(m)
}

pub fn identity4() -> Matrix4 {
    Matrix4(DMat4::IDENTITY)
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use approx::{assert_relative_eq, AbsDiffEq, RelativeEq};
    use crate::tuples::tuple;

    // wrap DVec4 so that we can define f64::relative_eq! for it
    #[derive(PartialEq)]
    struct DVec4Wrap(glam::f64::DVec4);

    impl AbsDiffEq for DVec4Wrap {
        type Epsilon = f64;

        fn default_epsilon() -> f64 {
            f64::default_epsilon()
        }
        fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
            self.0.abs_diff_eq(other.0, epsilon)
        }

    }
    impl RelativeEq for DVec4Wrap {
        fn default_max_relative() -> f64 {
            f64::default_max_relative()
        }

        fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
            f64::relative_eq(&self.0.x, &other.0.x, epsilon, max_relative)
        }
    }

    impl AbsDiffEq for Matrix4 {
        type Epsilon = f64;

        fn default_epsilon() -> f64 {
            f64::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
            self.0.abs_diff_eq(other.0, epsilon)
        }
    }

    impl RelativeEq for Matrix4 {
        fn default_max_relative() -> f64 {
            f64::default_max_relative()
        }

        fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
            DVec4Wrap::relative_eq(&DVec4Wrap(self.0.x_axis), &DVec4Wrap(other.0.x_axis), epsilon, max_relative) &&
                DVec4Wrap::relative_eq(&DVec4Wrap(self.0.y_axis), &DVec4Wrap(other.0.y_axis), epsilon, max_relative) &&
                DVec4Wrap::relative_eq(&DVec4Wrap(self.0.z_axis), &DVec4Wrap(other.0.z_axis), epsilon, max_relative) &&
                DVec4Wrap::relative_eq(&DVec4Wrap(self.0.w_axis), &DVec4Wrap(other.0.w_axis), epsilon, max_relative)
        }
    }

    #[test]
    fn default_matrix2_is_identity() {
        let A = Matrix2::default();

        for r in 0..2 {
            for c in 0..2 {
                assert_eq!(A.at(r, c), if r == c {1.0} else {0.0});
            }
        }
    }

    #[test]
    fn default_matrix3_is_identity() {
        let A = Matrix3::default();

        for r in 0..3 {
            for c in 0..3 {
                assert_eq!(A.at(r, c), if r == c {1.0} else {0.0});
            }
        }
    }

    #[test]
    fn default_matrix4_is_identity() {
        let A = Matrix4::default();

        for r in 0..4 {
            for c in 0..4 {
                assert_eq!(A.at(r, c), if r == c {1.0} else {0.0});
            }
        }
    }
/*
    #[test]
    fn matrix_from_vector() {
        std::vector<double> el = {
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
        };
        Matrix<3> A {el};
        EXPECT_EQ(A, matrix3x3({
            { 0.0,  1.0,   2.0},
            { 3.0,  4.0,   5.0},
            { 6.0,  7.0,   8.0},
        }));
    }
 */

    #[test]
    #[should_panic]
    fn invalid_access_0_4() {
        let M = Matrix4::new();
        M.at(0, 4);
    }

    #[test]
    #[should_panic]
    fn invalid_access_4_0() {
        let M = Matrix4::new();
        M.at(4, 0);
    }

    #[test]
    #[should_panic]
    fn invalid_access_4_4() {
        let M = Matrix4::new();
        M.at(4, 4);
    }

    // Constructing and inspecting a 4x4 matrix
    #[test]
    fn constructing_and_inspecting_4x4() {
        let _M = Matrix4::from_rows_array(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.5,  6.5,  7.5,  8.5],
            [ 9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]],
        );

        let M = matrix4(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.5,  6.5,  7.5,  8.5],
            [ 9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(M.at(0, 0), 1.0);
        assert_eq!(M.at(0, 3), 4.0);
        assert_eq!(M.at(1, 0), 5.5);
        assert_eq!(M.at(1, 2), 7.5);
        assert_eq!(M.at(2, 2), 11.0);
        assert_eq!(M.at(3, 0), 13.5);
        assert_eq!(M.at(3, 2), 15.5);
    }

    // Constructing and inspecting a 2x2 matrix
    #[test]
    fn constructing_and_inspecting_2x2() {
        let _M22 = Matrix2::from_rows_array(&[
            [ -3.0,  5.0],
            [  1.0, -2.0],
        ]);

        let M = matrix2(&[
            [ -3.0,  5.0],
            [  1.0, -2.0],
        ]);

        assert_eq!(M.at(0, 0), -3.0);
        assert_eq!(M.at(0, 1), 5.0);
        assert_eq!(M.at(1, 0), 1.0);
        assert_eq!(M.at(1, 1), -2.0);
    }

    // Constructing and inspecting a 3x3 matrix
    #[test]
    fn constructing_and_inspecting_3x3() {
        let _M33 = Matrix3::from_rows_array(&[
            [-3.0,  5.0,  0.0],
            [ 1.0, -2.0, -7.0],
            [ 0.0,  1.0,  1.0],
        ]);

        let M = matrix3(&[
            [ -3.0,  5.0,  0.0],
            [  1.0, -2.0, -7.0],
            [  0.0,  1.0,  1.0],
        ]);

        assert_eq!(M.at(0, 0), -3.0);
        assert_eq!(M.at(1, 1), -2.0);
        assert_eq!(M.at(2, 2), 1.0);
    }

    // Matrix equality with identical matrices
    #[test]
    fn equality_with_identical_matrices() {
        let A = matrix4(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.0,  6.0,  7.0,  8.0],
            [ 9.0,  8.0,  7.0,  6.0],
            [ 5.0,  4.0,  3.0,  2.0],
        ]);
        let B = matrix4(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.0,  6.0,  7.0,  8.0],
            [ 9.0,  8.0,  7.0,  6.0],
            [ 5.0,  4.0,  3.0,  2.0],
        ]);
        assert_eq!(A, B);
    }

    // Matrix equality with different matrices
    #[test]
    fn equality_with_different_matrices() {
        let A = matrix4(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.0,  6.0,  7.0,  8.0],
            [ 9.0,  8.0,  7.0,  6.0],
            [ 5.0,  4.0,  3.0,  2.0],
        ]);
        let B = matrix4(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 6.0,  7.0,  8.0,  9.0],
            [ 9.0,  8.0,  7.0,  6.0],
            [ 5.0,  4.0,  3.0,  2.0],
        ]);
        assert_ne!(A, B);
    }

    // Multiplying two matrices
    #[test]
    fn multiplying_two_matrices() {
        let A = matrix4(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.0,  6.0,  7.0,  8.0],
            [ 9.0,  8.0,  7.0,  6.0],
            [ 5.0,  4.0,  3.0,  2.0],
        ]);
        let B = matrix4(&[
            [-2.0,  1.0,  2.0,  3.0],
            [ 3.0,  2.0,  1.0, -1.0],
            [ 4.0,  3.0,  6.0,  5.0],
            [ 1.0,  2.0,  7.0,  8.0],
        ]);
        assert_eq!(A * B, matrix4(&[
            [20.0, 22.0,  50.0,  48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0,  46.0,  42.0],
        ]));
    }

    // A matrix multiplied by a tuple
    #[test]
    fn matrix_multipied_by_tuple() {
        let A = matrix4(&[
            [ 1.0,  2.0,  3.0,  4.0],
            [ 2.0,  4.0,  4.0,  2.0],
            [ 8.0,  6.0,  4.0,  1.0],
            [ 0.0,  0.0,  0.0,  1.0],
        ]);
        let b = tuple(1.0, 2.0, 3.0, 1.0);
        assert_eq!(A * b, tuple(18.0, 24.0, 33.0, 1.0));
    }

    // Multiplying a matrix by the identity matrix
    #[test]
    fn matrix_multiplied_by_identity() {
        let A = matrix4(&[
            [ 0.0,  1.0,  2.0,  4.0],
            [ 1.0,  2.0,  4.0,  8.0],
            [ 2.0,  4.0,  8.0, 16.0],
            [ 4.0,  8.0, 16.0, 32.0],
        ]);
        let I = identity4();
        assert_eq!(&A * &I, A);
        assert_eq!(I * &A, A);
    }

    // Multiplying the identity matrix by a tuple
    #[test]
    fn identity_multiplied_by_tuple() {
        let a = tuple(1.0, 2.0, 3.0, 4.0);
        let I = identity4();
        assert_eq!(I * &a, a);
    }

    // Transposing a matrix
    #[test]
    fn transposing_a_matrix() {
        let A = matrix4(&[
            [ 0.0,  9.0,  3.0,  0.0],
            [ 9.0,  8.0,  0.0,  8.0],
            [ 1.0,  8.0,  5.0,  3.0],
            [ 0.0,  0.0,  5.0,  8.0],
        ]);

        assert_eq!(A.transpose(), matrix4(&[
            [ 0.0,  9.0,   1.0,   0.0],
            [ 9.0,  8.0,   8.0,   0.0],
            [ 3.0,  0.0,   5.0,   5.0],
            [ 0.0,  8.0,   3.0,   8.0],
        ]));
    }

    // Transposing the identity matrix
    #[test]
    fn transposing_the_identity() {
        let I = identity4();
        assert_eq!(I.transpose(), I);
    }

    // Calculating the determinant of a 2x2 matrix
    #[test]
    fn calculate_determinant_2x2() {
        let A = matrix2(&[
            [ 1.0, 5.0],
            [-3.0, 2.0],
        ]);
        assert_eq!(A.determinant(), 17.0);
    }

/*
    // A submatrix of a 3x3 matrix is a 2x2 matrix
    #[test]
    fn submatrix_of_3x3() {
        auto A = matrix3x3({
            {  1.0,  5.0,  0.0},
            { -3.0,  2.0,  7.0},
            {  0.0,  6.0, -3.0},
        });

        EXPECT_EQ(submatrix(A, 0, 2), matrix2x2({
            {-3.0, 2.0},
            { 0.0, 6.0},
        }));
    }

    // A submatrix of a 4x4 matrix is a 3x3 matrix
    #[test]
    fn submatrix_of_4x4() {
        auto A = matrix4x4({
            {-6.0,  1.0,  1.0,  6.0},
            {-8.0,  5.0,  8.0,  6.0},
            {-1.0,  0.0,  8.0,  2.0},
            {-7.0,  1.0, -1.0,  1.0},
        });

        EXPECT_EQ(submatrix(A, 2, 1), matrix3x3({
            {-6.0, 1.0, 6.0},
            {-8.0, 8.0, 6.0},
            {-7.0,-1.0, 1.0},
        }));
    }

    // Calculating a minor of a 3x3 matrix
    #[test]
    fn calculate_minor_of_3x3() {
        auto A = matrix3x3({
            {  3.0,  5.0,  0.0},
            {  2.0, -1.0, -7.0},
            {  6.0, -1.0,  5.0},
        });
        auto B = submatrix(A, 1, 0);
        EXPECT_EQ(determinant(B), 25.0);
        EXPECT_EQ(minor(A, 1, 0), 25.0);
    }

    // Calculating a cofactor of a 3x3 matrix
    #[test]
    fn calculate_cofactor_of_3x3() {
        auto A = matrix3x3({
            {  3.0,  5.0,  0.0},
            {  2.0, -1.0, -7.0},
            {  6.0, -1.0,  5.0},
        });
        EXPECT_EQ(minor(A, 0, 0), -12.0);
        EXPECT_EQ(cofactor(A, 0, 0), -12.0);
        EXPECT_EQ(minor(A, 1, 0), 25.0);
        EXPECT_EQ(cofactor(A, 1, 0), -25.0);
    }
*/
    // Calculating the determinant of a 3x3 matrix
    #[test]
    fn calculate_determinant_of_3x3() {
        let A = matrix3(&[
            [  1.0,  2.0,  6.0],
            [ -5.0,  8.0, -4.0],
            [  2.0,  6.0,  4.0],
        ]);
        // assert_eq!(cofactor(A, 0, 0), 56.0);
        // assert_eq!(cofactor(A, 0, 1), 12.0);
        // assert_eq!(cofactor(A, 0, 2), -46.0);
        assert_eq!(A.determinant(), -196.0);
    }
    // Calculating the determinant of a 4x4 matrix
    #[test]
    fn calculate_determinant_of_4x4() {
        let A = matrix4(&[
            [-2.0, -8.0,  3.0,  5.0],
            [-3.0,  1.0,  7.0,  3.0],
            [ 1.0,  2.0, -9.0,  6.0],
            [-6.0,  7.0,  7.0, -9.0],
        ]);
        // assert_eq!(cofactor(A, 0, 0), 690.0);
        // assert_eq!(cofactor(A, 0, 1), 447.0);
        // assert_eq!(cofactor(A, 0, 2), 210.0);
        // assert_eq!(cofactor(A, 0, 3), 51.0);
        assert_eq!(A.determinant(), -4071.0);
    }

    // Testing an invertible matrix for invertibility
    #[test]
    fn invertible_matrix_is_invertible() {
        let A = matrix4(&[
            [ 6.0,  4.0,  4.0,  4.0],
            [ 5.0,  5.0,  7.0,  6.0],
            [ 4.0, -9.0,  3.0, -7.0],
            [ 9.0,  1.0,  7.0, -6.0],
        ]);
        assert_eq!(A.determinant(), -2120.0);
        assert!(A.is_invertible());
    }

    // Testing a noninvertible matrix for invertibility
    #[test]
    fn noninvertible_matrix_is_not_invertible() {
        let A = matrix4(&[
            [-4.0,  2.0, -2.0, -3.0],
            [ 9.0,  6.0,  2.0,  6.0],
            [ 0.0, -5.0,  1.0, -5.0],
            [ 0.0,  0.0,  0.0,  0.0],
        ]);
        assert_eq!(A.determinant(), 0.0);
        assert!(!A.is_invertible());
    }

    // Calculating the inverse of a matrix
    #[test]
    fn calculate_inverse_of_matrix() {
        let A = matrix4(&[
            [-5.0,  2.0,  6.0, -8.0],
            [ 1.0, -5.0,  1.0,  8.0],
            [ 7.0,  7.0, -6.0, -7.0],
            [ 1.0, -3.0,  7.0,  4.0],
        ]);
        let B = A.inverse();
        assert_eq!(A.determinant(), 532.0);
        //assert_eq!(cofactor(A, 2, 3), -160.0);
        assert_eq!(B.at(3, 2), -160.0 / 532.0);
        //assert_eq!(cofactor(A, 3, 2), 105.0);
        assert_eq!(B.at(2, 3), 105.0 / 532.0);
        assert_relative_eq!(B, matrix4(&[
            [ 0.21805,   0.45113,   0.24060,  -0.04511],
            [-0.80827,  -1.45677,  -0.44361,   0.52068],
            [-0.07895,  -0.22368,  -0.05263,   0.19737],
            [-0.52256,  -0.81391,  -0.30075,   0.30639],
        ]), epsilon=1e-5);
    }

    // Calculating the inverse of another matrix
    #[test]
    fn calculate_inverse_of_another_matrix() {
        let A = matrix4(&[
            [ 8.0, -5.0,  9.0,  2.0],
            [ 7.0,  5.0,  6.0,  1.0],
            [-6.0,  0.0,  9.0,  6.0],
            [-3.0,  0.0, -9.0, -4.0],
        ]);
        assert_relative_eq!(A.inverse(), matrix4(&[
            [-0.15385,  -0.15385,  -0.28205,  -0.53846],
            [-0.07692,   0.12308,   0.02564,   0.03077],
            [ 0.35897,   0.35897,   0.43590,   0.92308],
            [-0.69231,  -0.69231,  -0.76923,  -1.92308],
        ]), epsilon=1e-5);
    }

    // Calculating the inverse of a third matrix
    #[test]
    fn calculate_inverse_of_a_third_matrix() {
        let A = matrix4(&[
            [ 9.0,  3.0,  0.0,  9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0,  9.0,  6.0,  4.0],
            [-7.0,  6.0,  6.0,  2.0],
        ]);
        assert_relative_eq!(A.inverse(), matrix4(&[
            [-0.04074, -0.07778,  0.14444, -0.22222],
            [-0.07778,  0.03333,  0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926,  0.12963],
            [ 0.17778,  0.06667, -0.26667,  0.33333],
        ]), epsilon=1e-5);
    }

    // Multiplying a product by its inverse}
    #[test]
    fn multiply_product_by_inverse() {
        let A = matrix4(&[
            [ 3.0,  -9.0,   7.0,   3.0],
            [ 3.0,  -8.0,   2.0,  -9.0],
            [-4.0,   4.0,   4.0,   1.0],
            [-6.0,   5.0,  -1.0,   1.0],
        ]);
        let B = matrix4(&[
            [8.0,   2.0,  2.0,  2.0],
            [3.0,  -1.0,  7.0,  0.0],
            [7.0,   0.0,  5.0,  4.0],
            [6.0,  -2.0,  0.0,  5.0],
        ]);
        let C = &A * &B;
        assert_relative_eq!(C * B.inverse(), A, epsilon=1e-5);
    }
}
