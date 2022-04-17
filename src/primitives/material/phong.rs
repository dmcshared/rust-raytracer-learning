use crate::{
    gfx::primitives::{color::ColorRGBA, mix_modes::MixMode},
    primitives::{
        intersection::Intersection,
        light::{Light, Lights},
        ray::Ray,
    },
};

use super::Material;

#[derive(Debug, Clone, Copy)]
pub struct Phong {
    pub ambient: ColorRGBA,
    pub diffuse: ColorRGBA,
    pub specular: ColorRGBA,
    pub shininess: f64,
}

impl Default for Phong {
    fn default() -> Self {
        Self {
            ambient: ColorRGBA::new(0.1, 0.1, 0.1, 1.0),
            diffuse: ColorRGBA::new(0.9, 0.9, 0.9, 1.0),
            specular: ColorRGBA::new(0.9, 0.9, 0.9, 1.0),
            shininess: 0.0,
        }
    }
}

impl Material for Phong {
    fn render(
        &self,
        intersection: &Intersection,
        lights: &Lights,
    ) -> crate::gfx::primitives::color::ColorRGBA {
        let light_dot_normal =
            lights.light_effectiveness(Ray::new(intersection.world_pos, intersection.world_normal));

        let (diffuse, specular) = if light_dot_normal.3 <= 0.0 {
            (ColorRGBA::blank(), ColorRGBA::blank())
        } else {
            let reflectv = Ray::new(
                intersection.world_pos,
                intersection
                    .ray
                    .direction
                    .reflect_across(intersection.world_normal),
            );
            let reflect_dot_light = lights.light_effectiveness_exp(reflectv, self.shininess);

            let specular = if reflect_dot_light.3 <= 0.0 {
                ColorRGBA::blank()
            } else {
                let factor = reflect_dot_light;
                factor.mix(self.specular, MixMode::Mul)
            };

            (light_dot_normal.mix(self.diffuse, MixMode::Mul), specular)
        };

        // self.ambient + diffuse + specular
        // specular
        // diffuse

        self.ambient
            .mix(diffuse, MixMode::Alpha)
            .mix(specular, MixMode::Alpha)
    }
}

// Factory
impl Phong {
    pub fn with_ambient(&self, ambient: ColorRGBA) -> Self {
        Self { ambient, ..*self }
    }

    pub fn with_diffuse(&self, diffuse: ColorRGBA) -> Self {
        Self { diffuse, ..*self }
    }

    pub fn with_specular(&self, specular: ColorRGBA) -> Self {
        Self { specular, ..*self }
    }

    pub fn with_shininess(&self, shininess: f64) -> Self {
        Self { shininess, ..*self }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn default_phong_material() {
        let m = Phong::default();

        assert_eq!(m.ambient, ColorRGBA::new(0.1, 0.1, 0.1, 1.0));
        assert_eq!(m.diffuse, ColorRGBA::new(0.9, 0.9, 0.9, 1.0));
        assert_eq!(m.specular, ColorRGBA::new(0.9, 0.9, 0.9, 1.0));
        assert_eq!(m.shininess, 0.0);
    }

    #[test]
    fn phong_material_can_be_constructed_with_builder() {
        let ambient = ColorRGBA::new(0.05, 0.05, 0.05, 1.0);
        let diffuse = ColorRGBA::new(0.7, 0.7, 0.7, 1.0);
        let specular = ColorRGBA::new(0.95, 0.95, 0.95, 1.0);
        let shininess = 400.0;

        let m = Phong::default()
            .with_ambient(ambient)
            .with_diffuse(diffuse)
            .with_specular(specular)
            .with_shininess(shininess);

        assert_eq!(m.ambient, ambient);
        assert_eq!(m.diffuse, diffuse);
        assert_eq!(m.specular, specular);
        assert_eq!(m.shininess, shininess);
    }
}
