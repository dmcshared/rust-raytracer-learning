use crate::{gfx::primitives::color::ColorRGBA, util::Defaultable};

use super::Material;

#[derive(Debug, Clone, Copy)]
pub struct Phong {
    pub ambient: ColorRGBA,
    pub diffuse: ColorRGBA,
    pub specular: ColorRGBA,
    pub shininess: f64,
}

impl Defaultable for Phong {
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
        intersection: crate::primitives::intersection::Intersection,
    ) -> crate::gfx::primitives::color::ColorRGBA {
        todo!("Render the Phong material")
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
