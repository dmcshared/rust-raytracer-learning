use crate::{
    gfx::primitives::color::ColorRGBA,
    primitives::{intersection::Intersection, light::Lights, material::Material},
};

/// A simple ambient color material.
/// Use this file as a template for new materials.
#[derive(Debug)]
pub struct Ambient {
    color: ColorRGBA,
}

impl Ambient {
    pub fn new(color: ColorRGBA) -> Self {
        Self { color }
    }
}

impl Material for Ambient {
    fn render(&self, _intersection: &Intersection, _lights: &Lights) -> ColorRGBA {
        self.color
    }
}
