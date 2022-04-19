use crate::prelude::material::*;
use std::sync::Arc;

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
