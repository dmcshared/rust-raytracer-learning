use crate::util::fuzzy_comparison::FuzzyPartialEq;

pub type Matrix4f = Matrix<4, 4>;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const WIDTH: usize, const HEIGHT: usize> {
    data: [[f64; WIDTH]; HEIGHT],
}

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
}

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
}
