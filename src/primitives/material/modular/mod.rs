pub mod ambient;
pub mod combinators;
pub mod diffuse;
pub mod special;
pub mod specular;
pub mod util;

use std::sync::Arc;

use crate::{
    gfx::primitives::{color::ColorRGBA, mix_modes::MixMode},
    primitives::{intersection::Intersection, world_info::WorldInfo},
};

use super::Material;

#[derive(Debug)]
pub struct MaterialStack {
    pub materials: Vec<Arc<dyn Material>>,
}

impl MaterialStack {
    pub fn new(materials: Vec<Arc<dyn Material>>) -> Self {
        Self { materials }
    }
}

impl Material for MaterialStack {
    fn render(&self, intersection: &Intersection, world_info: Arc<WorldInfo>) -> ColorRGBA {
        self.materials
            .iter()
            .fold(ColorRGBA::new(1.0, 0.0, 1.0, 1.0), |acc, material| {
                acc.mix(
                    material.render(intersection, world_info.clone()),
                    MixMode::Alpha,
                )
            })
    }
}
