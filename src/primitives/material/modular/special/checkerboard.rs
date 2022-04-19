use crate::{
    gfx::primitives::color::ColorRGBA,
    primitives::{intersection::Intersection, light::Lights, material::Material},
};

/// A simple ambient color material.
/// Use this file as a template for new materials.
#[derive(Debug)]
pub struct CheckerBoard {
    color1: ColorRGBA,
    color2: ColorRGBA,
}

impl CheckerBoard {
    pub fn new(color1: ColorRGBA, color2: ColorRGBA) -> Self {
        Self { color1, color2 }
    }
}

impl Material for CheckerBoard {
    fn render(&self, intersection: &Intersection, _lights: &Lights) -> ColorRGBA {
        let x = (intersection.world_pos.0 .0 * 4.0) as i32;
        let y = (intersection.world_pos.0 .1 * 4.0) as i32;
        let z = (intersection.world_pos.0 .2 * 4.0) as i32;
        let g = (x + y + z) % 2;

        if g == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}
