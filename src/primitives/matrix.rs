use std::ops;

use crate::util::fuzzy_comparison::FuzzyPartialEq;

use super::three_part::{point::Point, vector::Vector, ThreePart};

pub type MatrixRow<const WIDTH: usize> = [f64; WIDTH];
pub type MatrixRaw<const WIDTH: usize, const HEIGHT: usize> = [MatrixRow<WIDTH>; HEIGHT];

pub type Matrix4f = Matrix<4, 4>;
pub type Matrix3f = Matrix<3, 3>;
pub type Matrix2f = Matrix<2, 2>;
pub type ColumnVector = Matrix<1, 4>;
pub type RowVector = Matrix<4, 1>;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const WIDTH: usize, const HEIGHT: usize> {
    // data: [[f64; WIDTH]; HEIGHT],
    data: MatrixRaw<WIDTH, HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize> ops::Index<usize> for Matrix<WIDTH, HEIGHT> {
    type Output = [f64; WIDTH];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ops::IndexMut<usize> for Matrix<WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Regular Impl
impl<const WIDTH: usize, const HEIGHT: usize> Matrix<WIDTH, HEIGHT> {
    fn new() -> Self {
        Self {
            data: [[0.0; WIDTH]; HEIGHT],
        }
    }

    fn new_with_data(data: [[f64; WIDTH]; HEIGHT]) -> Self {
        Self { data }
    }

    fn get_position(&self, x: usize, y: usize) -> f64 {
        self.data[y][x]
    }

    fn set_position(&mut self, x: usize, y: usize, value: f64) {
        self.data[y][x] = value;
    }

    fn transpose(&self) -> Matrix<HEIGHT, WIDTH> {
        let mut result = Matrix::<HEIGHT, WIDTH>::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.set_position(y, x, self.get_position(x, y));
            }
        }
        result
    }
}

// Square Matrices
impl<const SIZE: usize> Matrix<SIZE, SIZE> {
    fn identity() -> Self {
        let mut out = Self::new();

        for p in 0..SIZE {
            out.set_position(p, p, 1.0);
        }

        out
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> FuzzyPartialEq<Self> for Matrix<WIDTH, HEIGHT> {
    fn fuzzy_eq(self, other: Self) -> bool {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if !self.get_position(x, y).fuzzy_eq(other.get_position(x, y)) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const WIDTH: usize, const WIDTHHEIGHT: usize, const HEIGHT: usize>
    ops::Mul<Matrix<WIDTH, WIDTHHEIGHT>> for Matrix<WIDTHHEIGHT, HEIGHT>
{
    type Output = Matrix<WIDTH, HEIGHT>;

    fn mul(self, rhs: Matrix<WIDTH, WIDTHHEIGHT>) -> Self::Output {
        let mut m = Self::Output::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut sum = 0.0;

                for i in 0..WIDTHHEIGHT {
                    sum += self.get_position(i, y) * rhs.get_position(x, i);
                }

                m.set_position(x, y, sum);
            }
        }

        m
    }
}

// impl direct matrix * point and matrix * vector to avoid (dangerous?) conversions
impl ops::Mul<Point> for Matrix4f {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let mut out = self * ColumnVector::from(rhs);

        out[3][0] = 1.0;

        Point(out.into())
    }
}

impl ops::Mul<Vector> for Matrix4f {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let mut out = self * ColumnVector::from(rhs);

        out[3][0] = 0.0;

        Vector(out.into())
    }
}

impl From<ThreePart> for ColumnVector {
    fn from(threepart: ThreePart) -> Self {
        Self::new_with_data([[threepart.0], [threepart.1], [threepart.2], [threepart.3]])
    }
}
impl From<Point> for ColumnVector {
    fn from(p: Point) -> Self {
        Self::from(p.0)
    }
}
impl From<Vector> for ColumnVector {
    fn from(v: Vector) -> Self {
        Self::from(v.0)
    }
}

impl From<ColumnVector> for ThreePart {
    fn from(mat: ColumnVector) -> Self {
        ThreePart(
            mat.get_position(0, 0),
            mat.get_position(0, 1),
            mat.get_position(0, 2),
            mat.get_position(0, 3),
        )
    }
}
impl From<RowVector> for ThreePart {
    fn from(mat: RowVector) -> Self {
        ThreePart(
            mat.get_position(0, 0),
            mat.get_position(1, 0),
            mat.get_position(2, 0),
            mat.get_position(3, 0),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, assert_fuzzy_neq};

    use super::*;

    #[test]
    fn create_a_matrix_and_check_empty_initialized() {
        let matrix = Matrix4f::new();

        for y in 0..4 {
            for x in 0..4 {
                assert_eq!(matrix.get_position(x, y), 0.0);
            }
        }
    }

    #[test]
    fn create_an_identity_matrix() {
        let matrix = Matrix4f::identity();

        for p in 0..4 {
            assert_eq!(matrix.get_position(p, p), 1.0);
        }
    }

    #[test]
    fn create_a_matrix_with_data() {
        let matrix = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        for y in 0..4 {
            for x in 0..4 {
                assert_eq!(matrix.get_position(x, y), ((x + 1) + (y * 4)) as f64);
            }
        }
    }

    #[test]
    fn compare_two_matrices() {
        let matrix1 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let matrix2 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        // assert!(matrix1.fuzzy_eq(matrix2));
        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn compare_two_fuzzy_similar_matrices() {
        let matrix1 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.000005, 8.0],
            [9.0, 10.000002, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let matrix2 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_fuzzy_eq!(matrix1, matrix2);
    }

    #[test]
    fn compare_two_fuzzy_nonsimilar_matrices() {
        let matrix1 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.00005, 8.0],
            [9.0, 10.000002, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let matrix2 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_fuzzy_neq!(matrix1, matrix2);
    }

    #[test]
    fn multiply_two_4x4_matrices() {
        let matrix1 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let matrix2 = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let result = matrix1 * matrix2;

        assert_fuzzy_eq!(
            result,
            Matrix4f::new_with_data([
                [90.0, 100.0, 110.0, 120.0],
                [202.0, 228.0, 254.0, 280.0],
                [314.0, 356.0, 398.0, 440.0],
                [426.0, 484.0, 542.0, 600.0],
            ])
        );
    }

    #[test]
    fn multiply_2x3_matrix_with_3x1_matrix() {
        let matrix1 = Matrix::<3, 2>::new_with_data([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);

        let matrix2 = Matrix::<1, 3>::new_with_data([[7.0], [8.0], [9.0]]);

        let result = matrix1 * matrix2;

        assert_fuzzy_eq!(result, Matrix::<1, 2>::new_with_data([[50.0], [122.0],]));
    }

    #[test]
    fn check_index_works() {
        // remember get_position and set_position use x,y but index uses y,x

        let matrix = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(matrix.get_position(0, 0), matrix[0][0]);
        assert_eq!(matrix.get_position(1, 0), matrix[0][1]);
        assert_eq!(matrix.get_position(2, 0), matrix[0][2]);
        assert_eq!(matrix.get_position(3, 0), matrix[0][3]);
        assert_eq!(matrix.get_position(0, 1), matrix[1][0]);
        assert_eq!(matrix.get_position(1, 1), matrix[1][1]);
        assert_eq!(matrix.get_position(2, 1), matrix[1][2]);
        assert_eq!(matrix.get_position(3, 1), matrix[1][3]);
        assert_eq!(matrix.get_position(0, 2), matrix[2][0]);
        assert_eq!(matrix.get_position(1, 2), matrix[2][1]);
        assert_eq!(matrix.get_position(2, 2), matrix[2][2]);
        assert_eq!(matrix.get_position(3, 2), matrix[2][3]);
        assert_eq!(matrix.get_position(0, 3), matrix[3][0]);
        assert_eq!(matrix.get_position(1, 3), matrix[3][1]);
        assert_eq!(matrix.get_position(2, 3), matrix[3][2]);
        assert_eq!(matrix.get_position(3, 3), matrix[3][3]);
    }

    #[test]
    fn check_index_works_for_writing() {
        let mut matrix = Matrix4f::new();

        matrix[0][0] = 1.0;
        matrix[0][1] = 2.0;
        matrix[0][2] = 3.0;
        matrix[0][3] = 4.0;
        matrix[1][0] = 5.0;
        matrix[1][1] = 6.0;
        matrix[1][2] = 7.0;
        matrix[1][3] = 8.0;
        matrix[2][0] = 9.0;
        matrix[2][1] = 10.0;
        matrix[2][2] = 11.0;
        matrix[2][3] = 12.0;
        matrix[3][0] = 13.0;
        matrix[3][1] = 14.0;
        matrix[3][2] = 15.0;
        matrix[3][3] = 16.0;

        assert_fuzzy_eq!(
            matrix,
            Matrix4f::new_with_data([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ])
        );
    }

    #[test]
    fn multiply_identity_and_matrix() {
        let identity = Matrix4f::identity();
        let matrix = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let result = identity * matrix;

        assert_fuzzy_eq!(
            result,
            Matrix4f::new_with_data([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ])
        );
    }

    #[test]
    fn multiply_matrix_and_point_directly() {
        let point = Point::new(1.0, 2.0, 3.0);
        let matrix = Matrix4f::new_with_data([
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result: ColumnVector = matrix * ColumnVector::from(point);

        assert_eq!(ThreePart::from(result), ThreePart(2.0, 3.0, 1.0, 1.0));
    }

    #[test]
    fn multiply_matrix_and_point_indirectly() {
        let point = Point::new(1.0, 2.0, 3.0);
        let matrix = Matrix4f::new_with_data([
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result: Point = matrix * point;

        assert_eq!(result, Point::new(2.0, 3.0, 1.0));
    }

    #[test]
    fn try_translate_point_indirect() {
        let point = Point::new(1.0, 2.0, 3.0);
        let matrix = Matrix4f::new_with_data([
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 2.0],
            [0.0, 0.0, 1.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result: Point = matrix * point;

        assert_eq!(result, Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn try_translate_vector_indirect() {
        let point = Vector::new(1.0, 2.0, 3.0);
        let matrix = Matrix4f::new_with_data([
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 2.0],
            [0.0, 0.0, 1.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result: Vector = matrix * point;

        assert_eq!(result, Vector::new(1.0, 2.0, 3.0));
    }
}
