use std::sync::Arc;

use crate::{
    gfx::primitives::color::ColorRGBA,
    primitives::{intersection::Intersection, material::Material, world_info::WorldInfo},
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
    fn render(&self, _intersection: &Intersection, _world_info: Arc<WorldInfo>) -> ColorRGBA {
        self.color
    }
}
