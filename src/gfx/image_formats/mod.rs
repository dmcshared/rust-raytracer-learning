use super::{canvas::Canvas, primitives::color::ColorRGBA};

pub mod png;
pub mod ppm;

pub trait Image: Sized {
    fn as_bytes_header(&self) -> Vec<u8>;
    fn as_bytes(&self) -> Vec<u8>;
}
