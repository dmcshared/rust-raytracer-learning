use super::Matrix4f;
use crate::{
    assert_fuzzy_eq,
    primitives::{
        matrix::ColumnVector,
        rotation::Rotation,
        three_part::{point::Point, vector::Vector},
    },
    util::fuzzy_comparison::FuzzyPartialEq,
};

impl Matrix4f {
    pub fn translate(vec: Vector) -> Matrix4f {
        Matrix4f::translate_raw(vec.0 .0, vec.0 .1, vec.0 .2)
    }

    pub fn translate_raw(x: f64, y: f64, z: f64) -> Matrix4f {
        Self::new_with_data([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Matrix4f {
    pub fn scale(scalar: Point) -> Matrix4f {
        Matrix4f::scale_raw(scalar.0 .0, scalar.0 .1, scalar.0 .2)
    }

    pub fn scale_raw(x: f64, y: f64, z: f64) -> Matrix4f {
        Self::new_with_data([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale_uniform(scalar: f64) -> Matrix4f {
        Matrix4f::scale_raw(scalar, scalar, scalar)
    }
}

impl Matrix4f {
    pub fn cross_product(vec: Vector) -> Matrix4f {
        Matrix4f::cross_product_raw(vec.0 .0, vec.0 .1, vec.0 .2)
    }

    pub fn cross_product_raw(x: f64, y: f64, z: f64) -> Matrix4f {
        Self::new_with_data([
            [0.0, -z, y, 0.0],
            [z, 0.0, -x, 0.0],
            [-y, x, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Matrix4f {
    pub fn rotate_around(axis: Vector, angle: Rotation) -> Matrix4f {
        Matrix4f::rotate_around_raw(axis.0 .0, axis.0 .1, axis.0 .2, angle)
    }

    pub fn rotate_around_raw(x: f64, y: f64, z: f64, angle: Rotation) -> Matrix4f {
        assert_fuzzy_eq!(x * x + y * y + z * z, 1.0);

        let c = angle.val.cos();
        let s = angle.val.sin();
        let i = Self::identity();
        let t = 1.0 - c;
        let u = Vector::new(x, y, z);
        let cu = Matrix4f::cross_product(u);
        // with u = (x, y, z); o = u * u_T
        let o = ColumnVector::from(u) * ColumnVector::from(u).transpose();

        i * c + cu * s + o * t
    }

    pub fn rotate_around_x(angle: Rotation) -> Matrix4f {
        Matrix4f::rotate_around_x_raw(angle.val)
    }

    pub fn rotate_around_x_raw(angle: f64) -> Matrix4f {
        Matrix4f::new_with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, angle.cos(), -angle.sin(), 0.0],
            [0.0, angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_around_y(angle: Rotation) -> Matrix4f {
        Matrix4f::rotate_around_y_raw(angle.val)
    }

    pub fn rotate_around_y_raw(angle: f64) -> Matrix4f {
        Matrix4f::new_with_data([
            [angle.cos(), 0.0, angle.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-angle.sin(), 0.0, angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_around_z(angle: Rotation) -> Matrix4f {
        Matrix4f::rotate_around_z_raw(angle.val)
    }

    pub fn rotate_around_z_raw(angle: f64) -> Matrix4f {
        Matrix4f::new_with_data([
            [angle.cos(), -angle.sin(), 0.0, 0.0],
            [angle.sin(), angle.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn fix_transform(&self) -> Matrix4f {
        let mut mat = *self;
        mat.data[3][0] = 0.0;
        mat.data[3][1] = 0.0;
        mat.data[3][2] = 0.0;
        mat.data[3][3] = 1.0;
        mat
    }

    // pub fn rotate_around_x( angle: Rotation) -> Matrix4f {
    //     Matrix4f::rotate_around(Vector::new(1.0, 0.0, 0.0), angle)
    // }

    // pub fn rotate_around_y( angle: Rotation) -> Matrix4f {
    //     Matrix4f::rotate_around(Vector::new(0.0, 1.0, 0.0), angle)
    // }

    // pub fn rotate_around_z( angle: Rotation) -> Matrix4f {
    //     Matrix4f::rotate_around(Vector::new(0.0, 0.0, 1.0), angle)
    // }
}

impl Matrix4f {
    pub fn shear(x: f64, y: f64, z: f64) -> Matrix4f {
        Matrix4f::shear_raw(x * y, x * z, y * x, y * z, z * x, z * y)
    }

    pub fn shear_raw(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4f {
        Self::new_with_data([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::{
        matrix::ColumnVector, rotation::degrees::Degree, three_part::point::Point,
    };

    use super::*;

    #[test]
    fn ensure_transforms_are_invertable() {
        assert!(Matrix4f::translate(Vector::new(1.0, 2.0, 3.0)).is_invertible());
        assert!(Matrix4f::scale(Point::new(2.0, 3.0, 4.0)).is_invertible());
        assert!(!Matrix4f::scale(Point::new(0.0, 0.0, 0.0)).is_invertible());
    }

    #[test]
    fn create_and_apply_raw_translate_matrix_to_point() {
        let result = Matrix4f::translate_raw(1.0, 2.0, 3.0);
        let test_point = Point::new(4.0, 5.0, 6.0);

        let expected_point = Point::new(5.0, 7.0, 9.0);

        let transformed_point = result * ColumnVector::from(test_point);

        assert_eq!(
            transformed_point.data,
            ColumnVector::from(expected_point).data
        );
    }

    #[test]
    fn create_and_try_apply_raw_translate_matrix_to_vector() {
        let result = Matrix4f::translate_raw(1.0, 2.0, 3.0);
        let test_point = Vector::new(4.0, 5.0, 6.0);

        let expected_point = Vector::new(4.0, 5.0, 6.0);

        let transformed_point = result * ColumnVector::from(test_point);

        assert_eq!(
            transformed_point.data,
            ColumnVector::from(expected_point).data
        );
    }

    #[test]
    fn create_and_apply_translate_to_point() {
        let result = Matrix4f::translate(Vector::new(1.0, 2.0, 3.0));
        let test_point = Point::new(4.0, 5.0, 6.0);

        let expected_point = Point::new(5.0, 7.0, 9.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn create_and_apply_inverse_translate_to_point() {
        let result = Matrix4f::translate(Vector::new(1.0, 2.0, 3.0))
            .inverse()
            .unwrap();
        let test_point = Point::new(4.0, 5.0, 6.0);

        let expected_point = Point::new(3.0, 3.0, 3.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn create_and_apply_general_rotate_x_to_vector() {
        let result = Matrix4f::rotate_around(Vector::new(1.0, 0.0, 0.0), Degree(90.0).into());
        let test_point = Vector::new(0.0, 1.0, 0.0);

        let expected_point = Vector::new(0.0, 0.0, 1.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn create_and_apply_rotate_x_to_vector() {
        let result = Matrix4f::rotate_around_x(Degree(90.0).into());
        let test_point = Vector::new(0.0, 1.0, 0.0);

        let expected_point = Vector::new(0.0, 0.0, 1.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn create_and_apply_general_rotate_y_to_vector() {
        let result = Matrix4f::rotate_around(Vector::new(0.0, 1.0, 0.0), Degree(90.0).into());
        let test_point = Vector::new(1.0, 0.0, 0.0);

        let expected_point = Vector::new(0.0, 0.0, -1.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn create_and_apply_rotate_y_to_vector() {
        let result = Matrix4f::rotate_around_y(Degree(90.0).into());
        let test_point = Vector::new(1.0, 0.0, 0.0);

        let expected_point = Vector::new(0.0, 0.0, -1.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn create_and_apply_general_rotate_z_to_vector() {
        let result = Matrix4f::rotate_around(Vector::new(0.0, 0.0, 1.0), Degree(90.0).into());
        let test_point = Vector::new(0.0, 1.0, 0.0);

        let expected_point = Vector::new(-1.0, 0.0, 0.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn create_and_apply_rotate_z_to_vector() {
        let result = Matrix4f::rotate_around_z(Degree(90.0).into());
        let test_point = Vector::new(0.0, 1.0, 0.0);

        let expected_point = Vector::new(-1.0, 0.0, 0.0);

        let transformed_point = result * test_point;

        assert_eq!(transformed_point, expected_point);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix4f::shear_raw(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix4f::shear_raw(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix4f::shear_raw(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix4f::shear_raw(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix4f::shear_raw(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix4f::shear_raw(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformation_are_applied_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix4f::rotate_around_x(Degree(90.0).into());
        let b = Matrix4f::scale_raw(5.0, 5.0, 5.0);
        let c = Matrix4f::translate_raw(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_fuzzy_eq!(p2, Point::new(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_fuzzy_eq!(p3, Point::new(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_fuzzy_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix4f::rotate_around_x(Degree(90.0).into());
        let b = Matrix4f::scale_raw(5.0, 5.0, 5.0);
        let c = Matrix4f::translate_raw(10.0, 5.0, 7.0);

        let transform = c * b * a;
        assert_fuzzy_eq!(transform * p, Point::new(15.0, 0.0, 7.0));
    }
}
