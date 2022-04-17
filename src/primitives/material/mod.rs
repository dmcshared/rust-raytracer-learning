use crate::gfx::primitives::color::ColorRGBA;

use super::intersection::Intersection;

pub trait Material {
    fn render(&self, intersection: Intersection) -> ColorRGBA;
}
