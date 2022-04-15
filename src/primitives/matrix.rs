pub type Matrix4f = Matrix<4, 4>;

pub struct Matrix<const WIDTH: usize, const HEIGHT: usize> {
    pub data: [[f64; HEIGHT]; WIDTH],
}

impl<const WIDTH: usize, const HEIGHT: usize> Matrix<WIDTH, HEIGHT> {}
