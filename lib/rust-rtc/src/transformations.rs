// Chapter 4: Transformations

use crate::matrices::{matrix4, Matrix4};
use crate::tuples::{cross, normalize, Point, Vector};

#[rustfmt::skip]
pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    matrix4(&[
        [1.0, 0.0, 0.0,   x],
        [0.0, 1.0, 0.0,   y],
        [0.0, 0.0, 1.0,   z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    matrix4(&[
        [  x, 0.0, 0.0, 0.0],
        [0.0,   y, 0.0, 0.0],
        [0.0, 0.0,   z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn rotation_x(radians: f64) -> Matrix4 {
    let cos_r = f64::cos(radians);
    let sin_r = f64::sin(radians);
    matrix4(&[
        [1.0,   0.0,    0.0, 0.0],
        [0.0, cos_r, -sin_r, 0.0],
        [0.0, sin_r,  cos_r, 0.0],
        [0.0,   0.0,    0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn rotation_y(radians: f64) -> Matrix4 {
    let cos_r = f64::cos(radians);
    let sin_r = f64::sin(radians);
    matrix4(&[
        [ cos_r, 0.0, sin_r, 0.0],
        [   0.0, 1.0,   0.0, 0.0],
        [-sin_r, 0.0, cos_r, 0.0],
        [   0.0, 0.0,   0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn rotation_z(radians: f64) -> Matrix4 {
    let cos_r = f64::cos(radians);
    let sin_r = f64::sin(radians);
    matrix4(&[
        [cos_r, -sin_r, 0.0, 0.0],
        [sin_r,  cos_r, 0.0, 0.0],
        [  0.0,    0.0, 1.0, 0.0],
        [  0.0,    0.0, 0.0, 1.0],
    ])
}

#[rustfmt::skip]
pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
    matrix4(&[
        [1.0,  xy,  xz, 0.0],
        [ yx, 1.0,  yz, 0.0],
        [ zx,  zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn view_transform(from: &Point, to: &Point, up: &Vector) -> Matrix4 {
    let forward = normalize(&(to - from));
    let upn = normalize(up);
    let left = cross(&forward, &upn);
    let true_up = cross(&left, &forward);

    #[rustfmt::skip]
    let orientation = matrix4(&[
        [    left.x(),     left.y(),     left.z(), 0.0],
        [ true_up.x(),  true_up.y(),  true_up.z(), 0.0],
        [-forward.x(), -forward.y(), -forward.z(), 0.0],
        [         0.0,          0.0,          0.0, 1.0],
    ]);

    orientation * translation(-from.x(), -from.y(), -from.z())
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use crate::matrices::{identity4, inverse};
    use crate::tuples::{point, vector};
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    // Multiplying by a translation matrix
    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(2.0, 1.0, 7.0));
    }

    // Multiplying by the inverse of a translation matrix
    #[test]
    fn multiplying_by_inverse_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = inverse(&transform);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, point(-8.0, 7.0, 3.0));
    }

    // Translation does not affect vectors
    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    // A scaling matrix applied to a point
    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, point(-8.0, 18.0, 32.0));
    }

    // A scaling matrix applied to a vector
    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, vector(-8.0, 18.0, 32.0));
    }

    // Multiplying by the inverse of a scaling matrix
    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = inverse(&transform);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, vector(-2.0, 2.0, 2.0));
    }

    // Reflection is scaling by a negative value
    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(-2.0, 3.0, 4.0));
    }

    // Rotating a point around the x axis
    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_relative_eq!(
            half_quarter * p,
            point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
        assert_relative_eq!(full_quarter * p, point(0.0, 0.0, 1.0));
    }

    // The inverse of an x-rotation rotates in the opposite direction
    #[test]
    fn inverse_rotate_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = inverse(&half_quarter);
        assert_relative_eq!(
            inv * p,
            point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    // Rotating a point around the y axis
    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_relative_eq!(
            half_quarter * p,
            point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
        );
        assert_relative_eq!(full_quarter * p, point(1.0, 0.0, 0.0));
    }

    // Rotating a point around the z axis
    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_relative_eq!(
            half_quarter * p,
            point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0)
        );
        assert_relative_eq!(full_quarter * p, point(-1.0, 0.0, 0.0));
    }

    // A shearing transformation moves x in proportion to y
    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(5.0, 3.0, 4.0));
    }

    // A shearing transformation moves x in proportion to z
    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(6.0, 3.0, 4.0));
    }

    // A shearing transformation moves y in proportion to x
    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 5.0, 4.0));
    }

    // A shearing transformation moves y in proportion to z
    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 7.0, 4.0));
    }

    // A shearing transformation moves z in proportion to x
    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 3.0, 6.0));
    }

    // A shearing transformation moves z in proportion to y
    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 3.0, 7.0));
    }

    // Individual transformations are applied in sequence
    #[test]
    fn individual_transformations_applied_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let A = rotation_x(PI / 2.0);
        let B = scaling(5.0, 5.0, 5.0);
        let C = translation(10.0, 5.0, 7.0);

        let p2 = A * p;
        assert_relative_eq!(p2, point(1.0, -1.0, 0.0), epsilon = 1e-6);

        let p3 = B * p2;
        assert_relative_eq!(p3, point(5.0, -5.0, 0.0), epsilon = 1e-6);

        let p4 = C * p3;
        assert_relative_eq!(p4, point(15.0, 0.0, 7.0), epsilon = 1e-6);
    }

    // Chained transformations must be applied in reverse order
    #[test]
    fn chained_transformations_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let A = rotation_x(PI / 2.0);
        let B = scaling(5.0, 5.0, 5.0);
        let C = translation(10.0, 5.0, 7.0);

        let T = C * B * A;
        assert_relative_eq!(T * p, point(15.0, 0.0, 7.0));
    }

    // Fluent API
    #[test]
    fn fluent_api() {
        let p = point(1.0, 0.0, 1.0);

        let M = identity4()
            .then(&rotation_x(PI / 2.0))
            .then(&scaling(5.0, 5.0, 5.0))
            .then(&translation(10.0, 5.0, 7.0));

        assert_eq!(M * p, point(15.0, 0.0, 7.0));

        let M2 = identity4()
            .then(&rotation_x(PI / 2.0))
            .then(&scaling(3.0, 3.0, 3.0))
            .then(&translation(10.0, 5.0, 7.0));

        assert_eq!(M2 * p, point(13.0, 2.0, 7.0));
    }

    // The transformation matrix for the default orientation
    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, identity4());
    }

    // A view transformation matrix looking in positive Z direction
    #[test]
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    // The view transformation moves the world
    #[test]
    fn view_transformation_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    // An arbitrary view transformation
    #[test]
    fn arbitrary_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        #[rustfmt::skip]
        assert_relative_eq!(t, matrix4(&[
                [ -0.50709, 0.50709,  0.67612, -2.36643],
                [  0.76772, 0.60609,  0.12122, -2.82843],
                [ -0.35857, 0.59761, -0.71714,  0.00000],
                [  0.00000, 0.00000,  0.00000,  1.00000],
        ]), epsilon=1e-5);
    }
}
