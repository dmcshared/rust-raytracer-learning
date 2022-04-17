pub mod directional_light;
pub mod point_light;

use crate::gfx::primitives::color::ColorRGBA;

use super::ray::Ray;

pub trait Light {
    fn light_effectiveness(&self, r: Ray) -> f64;
    fn get_color(&self) -> ColorRGBA;
}

// for phong you would need to do light_effectiveness(r) * light_effectiveness(r.reflect_over(n))
