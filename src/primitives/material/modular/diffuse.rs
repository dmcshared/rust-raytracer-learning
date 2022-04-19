use std::sync::Arc;

use crate::{
    gfx::primitives::{color::ColorRGBA, mix_modes::MixMode},
    primitives::{intersection::Intersection, material::Material, ray::Ray, world_info::WorldInfo},
};

/// A simple ambient color material.
/// Use this file as a template for new materials.
#[derive(Debug)]
pub struct Diffuse {
    color: ColorRGBA,
}

impl Diffuse {
    pub fn new(color: ColorRGBA) -> Self {
        Self { color }
    }
}

impl Material for Diffuse {
    fn render(&self, intersection: &Intersection, world_info: Arc<WorldInfo>) -> ColorRGBA {
        let light_dot_normal = world_info
            .lights
            .light_effectiveness(Ray::new(intersection.world_pos, intersection.world_normal));

        if light_dot_normal.3 <= 0.0 {
            ColorRGBA::blank()
        } else {
            light_dot_normal.mix(self.color, MixMode::Mul)
        }
    }
}
