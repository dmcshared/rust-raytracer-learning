pub mod transform;

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
    pub fn new() -> Self {
        Self {
            data: [[0.0; WIDTH]; HEIGHT],
        }
    }

    pub fn new_with_data(data: [[f64; WIDTH]; HEIGHT]) -> Self {
        Self { data }
    }

    pub fn get_position(&self, x: usize, y: usize) -> f64 {
        self.data[y][x]
    }

    pub fn set_position(&mut self, x: usize, y: usize, value: f64) {
        self.data[y][x] = value;
    }

    pub fn transpose(&self) -> Matrix<HEIGHT, WIDTH> {
        let mut result = Matrix::<HEIGHT, WIDTH>::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.set_position(y, x, self.get_position(x, y));
            }
        }
        result
    }

    pub fn contiguous_submatrix<const NWIDTH: usize, const NHEIGHT: usize>(
        &self,
        x: usize,
        y: usize,
    ) -> Matrix<NWIDTH, NHEIGHT> {
        assert!((x + NWIDTH) <= WIDTH);
        assert!((y + NHEIGHT) <= HEIGHT);

        let mut result = Matrix::<NWIDTH, NHEIGHT>::new();
        for dy in 0..NHEIGHT {
            for dx in 0..NWIDTH {
                result.set_position(dx, dy, self.get_position(x + dx, y + dy));
            }
        }
        result
    }

    pub fn submatrix<const NWIDTH: usize, const NHEIGHT: usize>(
        &self,
        row: usize,
        column: usize,
    ) -> Matrix<NWIDTH, NHEIGHT> {
        assert!(WIDTH - NWIDTH == 1);
        assert!(HEIGHT - NHEIGHT == 1);

        let mut result = Matrix::<NWIDTH, NHEIGHT>::new();
        for dy in 0..NHEIGHT {
            for dx in 0..NWIDTH {
                result.set_position(
                    dx,
                    dy,
                    self.get_position(dx + ((dx >= column) as usize), dy + ((dy >= row) as usize)),
                );
            }
        }
        result
    }

    // pub fn minor<const NWIDTH: usize, const NHEIGHT: usize>(
    //     &self,
    //     row: usize,
    //     column: usize,
    // ) -> f64 {
    //     self.submatrix::<NWIDTH, NHEIGHT>(row, column).determinant()
    // }
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

impl Matrix4f {
    pub fn determinant(&self) -> f64 {
        let mut determinant: f64 = 0.0;
        for column in 0..4 {
            determinant += self.cofactor(0, column) * self[0][column];
        }

        determinant
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix::<3, 3>(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            // Even value
            minor
        } else {
            -minor
        }
    }

    pub fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(0.0)
    }

    pub fn inverse(&self) -> Option<Matrix4f> {
        let deter = self.determinant();

        if deter == 0.0 {
            return None;
        }

        let mut out = Matrix4f::new();

        for row in 0..4 {
            for column in 0..4 {
                out.set_position(row, column, self.cofactor(row, column) / deter);
            }
        }

        Some(out)
    }
}
impl Matrix3f {
    pub fn determinant(&self) -> f64 {
        let mut determinant: f64 = 0.0;
        for column in 0..3 {
            determinant += self.cofactor(0, column) * self[0][column];
        }

        determinant
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix::<2, 2>(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            // Even value
            minor
        } else {
            -minor
        }
    }

    pub fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(0.0)
    }
}
impl Matrix2f {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    pub fn is_invertible(&self) -> bool {
        !self.determinant().fuzzy_eq(0.0)
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

impl<const WIDTH: usize, const HEIGHT: usize> ops::Mul<f64> for Matrix<WIDTH, HEIGHT> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut m = Self::Output::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                m.set_position(x, y, self.get_position(x, y) * rhs);
            }
        }

        m
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ops::Div<f64> for Matrix<WIDTH, HEIGHT> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut m = Self::Output::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                m.set_position(x, y, self.get_position(x, y) / rhs);
            }
        }

        m
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ops::Add<Matrix<WIDTH, HEIGHT>>
    for Matrix<WIDTH, HEIGHT>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut m = Self::Output::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                m.set_position(x, y, self.get_position(x, y) + rhs.get_position(x, y));
            }
        }

        m
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ops::Sub<Matrix<WIDTH, HEIGHT>>
    for Matrix<WIDTH, HEIGHT>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut m = Self::Output::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                m.set_position(x, y, self.get_position(x, y) - rhs.get_position(x, y));
            }
        }

        m
    }
}

// impl direct matrix * point and matrix * vector to avoid (dangerous?) conversions
impl ops::Mul<Point> for Matrix4f {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let out = self * ColumnVector::from(rhs);

        // out[3][0] = 1.0;
        assert_eq!(out[3][0], 1.0);

        Point(out.into())
    }
}

impl ops::Mul<Vector> for Matrix4f {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let out = self * ColumnVector::from(rhs);

        // out[3][0] = 0.0;
        assert_eq!(out[3][0], 0.0);

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
        assert!(mat.get_position(0, 3) == 1.0 || mat.get_position(0, 3) == 0.0);

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
        assert!(mat.get_position(0, 3) == 1.0 || mat.get_position(0, 3) == 0.0);

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
    fn transpose_4x4_matrix() {
        let matrix = Matrix4f::new_with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let transposed = matrix.transpose();

        for y in 0..4 {
            for x in 0..4 {
                assert_eq!(transposed.get_position(x, y), matrix.get_position(y, x));
            }
        }
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

    #[test]
    fn determinant_of_2x2_matrix() {
        let matrix = Matrix2f::new_with_data([[1.0, 5.0], [-3.0, 2.0]]);

        assert_eq!(matrix.determinant(), 17.0);
    }

    #[test]
    fn get_2x2_submatrix_of_3x3() {
        let matrix = Matrix3f::new_with_data([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

        let submatrix = matrix.submatrix::<2, 2>(0, 2);

        assert_fuzzy_eq!(
            submatrix,
            Matrix2f::new_with_data([[-3.0, 2.0], [0.0, 6.0]])
        );
    }

    #[test]
    fn get_3x3_submatrix_of_4x4() {
        let matrix = Matrix4f::new_with_data([
            [1.0, 5.0, 0.0, 0.0],
            [-3.0, 2.0, 7.0, 0.0],
            [0.0, 6.0, -3.0, 0.0],
            [0.0, 0.0, 0.0, -2.0],
        ]);

        let submatrix = matrix.submatrix::<3, 3>(0, 0);

        assert_fuzzy_eq!(
            submatrix,
            Matrix3f::new_with_data([[2.0, 7.0, 0.0], [6.0, -3.0, 0.0], [0.0, 0.0, -2.0],])
        );
    }

    #[test]
    fn calculate_minor_of_3x3() {
        let m = Matrix3f::new_with_data([
            //
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0],
        ]);

        let sub = m.submatrix::<2, 2>(1, 0);
        let determinant = sub.determinant();
        let minor = m.minor(1, 0);

        assert_eq!(25.0, determinant);
        assert_eq!(25.0, minor);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let m = Matrix::new_with_data([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let minor1 = m.minor(0, 0);
        let minor2 = m.minor(1, 0);

        let cofactor1 = m.cofactor(0, 0);
        let cofactor2 = m.cofactor(1, 0);

        assert_fuzzy_eq!(-12.0, minor1);
        assert_fuzzy_eq!(-12.0, cofactor1);
        assert_fuzzy_eq!(25.0, minor2);
        assert_fuzzy_eq!(-25.0, cofactor2);
    }

    #[test]
    fn calculate_the_minor_of_a_3x3_matrix() {
        let m = Matrix3f::new_with_data([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let sub = m.submatrix::<2, 2>(1, 0);
        let determinant = sub.determinant();
        let minor = m.minor(1, 0);

        assert_fuzzy_eq!(25.0, determinant);
        assert_fuzzy_eq!(25.0, minor);
    }

    #[test]
    fn calculate_the_determinant_of_a_3x3_matrix() {
        let m = Matrix::new_with_data([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        let cofactor00 = m.cofactor(0, 0);
        let cofactor01 = m.cofactor(0, 1);
        let cofactor02 = m.cofactor(0, 2);

        let determinant = m.determinant();

        assert_fuzzy_eq!(56.0, cofactor00);
        assert_fuzzy_eq!(12.0, cofactor01);
        assert_fuzzy_eq!(-46.0, cofactor02);

        assert_fuzzy_eq!(-196.0, determinant);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let m = Matrix::new_with_data([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        let cofactor00 = m.cofactor(0, 0);
        let cofactor01 = m.cofactor(0, 1);
        let cofactor02 = m.cofactor(0, 2);
        let cofactor03 = m.cofactor(0, 3);

        let determinant = m.determinant();

        assert_fuzzy_eq!(690.0, cofactor00);
        assert_fuzzy_eq!(447.0, cofactor01);
        assert_fuzzy_eq!(210.0, cofactor02);
        assert_fuzzy_eq!(51.0, cofactor03);

        assert_fuzzy_eq!(-4071.0, determinant);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let m = Matrix::new_with_data([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        let determinant = m.determinant();

        assert_fuzzy_eq!(-2120.0, determinant);
        assert!(m.is_invertible());
    }

    #[test]
    fn testing_an_noninvertible_matrix_for_invertibility() {
        let m = Matrix::new_with_data([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        let determinant = m.determinant();

        assert_fuzzy_eq!(0.0, determinant);
        assert!(!m.is_invertible());
    }

    #[test]
    fn inverse_matrix() {
        let m = Matrix::new_with_data([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let determinant = m.determinant();
        let cofactor23 = m.cofactor(2, 3);
        let cofactor32 = m.cofactor(3, 2);

        let expected_result = Matrix::new_with_data([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        let actual_result = m.inverse().unwrap();

        assert_fuzzy_eq!(532.0, determinant);
        assert_fuzzy_eq!(-160.0, cofactor23);
        assert_fuzzy_eq!(-160.0 / 532.0, actual_result[3][2]);
        assert_fuzzy_eq!(105.0, cofactor32);
        assert_fuzzy_eq!(105.0 / 532.0, actual_result[2][3]);
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn matrix_times_inverted_matrix_should_be_identity() {
        let m = Matrix::new_with_data([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let inverse = m.inverse().unwrap();

        let identity = m * inverse;

        assert_fuzzy_eq!(identity, Matrix4f::identity());
    }
}
