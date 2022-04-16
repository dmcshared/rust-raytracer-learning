use crate::primitives::three_part::{point::Point, vector::Vector};

use super::Matrix4f;

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

#[cfg(test)]
mod tests {
    use crate::primitives::{matrix::ColumnVector, three_part::point::Point};

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
}
