use crate::{
    gfx::primitives::{color::ColorRGBA, mix_modes::MixMode},
    primitives::{
        intersection::Intersection,
        light::{Light, Lights},
        material::Material,
        ray::Ray,
    },
};

/// A simple ambient color material.
/// Use this file as a template for new materials.
#[derive(Debug)]
pub struct Specular {
    color: ColorRGBA,
    shininess: f64,
}

impl Specular {
    pub fn new(color: ColorRGBA, shininess: f64) -> Self {
        Self { color, shininess }
    }
}

impl Material for Specular {
    fn render(&self, intersection: &Intersection, lights: &Lights) -> ColorRGBA {
        let reflectv = Ray::new(
            intersection.world_pos,
            intersection
                .ray
                .direction
                .reflect_across(intersection.world_normal),
        );
        let reflect_dot_light = lights.light_effectiveness_exp(reflectv, self.shininess);
        // let reflect_dot_light = lights.light_effectiveness(reflectv).powf(self.shininess);

        if reflect_dot_light.3 <= 0.0 {
            ColorRGBA::blank()
        } else {
            reflect_dot_light.mix(self.color, MixMode::Mul)
        }
    }
}
