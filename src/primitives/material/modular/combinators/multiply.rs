use std::sync::Arc;

use crate::{
    gfx::primitives::{color::ColorRGBA, mix_modes::MixMode},
    primitives::{intersection::Intersection, material::Material, world_info::WorldInfo},
};

#[derive(Debug)]
pub struct Multiply {
    pub materials: Vec<Arc<dyn Material>>,
}

impl Multiply {
    pub fn new(materials: Vec<Arc<dyn Material>>) -> Self {
        Self { materials }
    }
}

impl Material for Multiply {
    fn render(&self, intersection: &Intersection, world_info: Arc<WorldInfo>) -> ColorRGBA {
        self.materials
            .iter()
            .fold(ColorRGBA::new(1.0, 1.0, 1.0, 1.0), |acc, material| {
                acc.mix(
                    material.render(intersection, world_info.clone()),
                    MixMode::Mul,
                )
            })
    }
}
