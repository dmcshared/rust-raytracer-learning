use super::primitives::color::ColorRGBA;

pub mod ppm;

pub trait Image: Sized {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn color_at(&self, x: usize, y: usize) -> ColorRGBA;

    fn set_width(&mut self, width: usize);
    fn set_height(&mut self, height: usize);
    fn set_color_at(&mut self, x: usize, y: usize, color: ColorRGBA);

    fn as_bytes_header(&self) -> Vec<u8>;
    fn as_bytes(&self) -> Vec<u8>;
}
